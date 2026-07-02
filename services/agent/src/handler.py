"""Dual-mode entry point for the agent service (ADR-004).

- Lambda: Mangum wraps the FastAPI app.
- Local dev: uvicorn on :3003.
"""

from __future__ import annotations

import asyncio
import json
import logging
import os
import time
import uuid
from collections.abc import AsyncGenerator
from typing import Any

import boto3
import botocore.config
from fastapi import FastAPI, HTTPException, Request
from fastapi.responses import JSONResponse, StreamingResponse
from mangum import Mangum

from agent.agent import AgentDeps, convert_html_to_pdf, get_agent, upload_pdf_and_link
from agent.preground import fetch_resume, match_keywords
from job_store import append_event, complete_job, create_job, fail_job, get_job
from linkedin_oauth import _restore_token, load_linkedin_credentials
from linkedin_oauth import router as linkedin_router
from models import (
    AgentEvent,
    CoverLetterJobCreateResponse,
    CoverLetterJobStatus,
    CoverLetterRequest,
    CoverLetterResponse,
)

logger = logging.getLogger(__name__)

# ── Rate limiter (2/day/IP, mirrors ask.rs pattern) ──────────────────────────

AGENT_RATE_LIMIT = int(os.environ.get("AGENT_RATE_LIMIT", "10"))
_RATE_WINDOW = 86400  # 24 hours
_rate_map: dict[str, dict[str, Any]] = {}
_background_tasks: set[asyncio.Task[None]] = set()


def _check_rate_limit(ip: str) -> bool:
    now = time.monotonic()
    entry = _rate_map.get(ip)
    if entry is None or now - entry["start"] >= _RATE_WINDOW:
        _rate_map[ip] = {"count": 1, "start": now}
        return True
    if entry["count"] >= AGENT_RATE_LIMIT:
        return False
    entry["count"] += 1
    return True


def _extract_client_ip(request: Request) -> str:
    if ip := request.headers.get("x-apigw-source-ip", "").strip():
        return ip
    if (forwarded := request.headers.get("x-forwarded-for", "")) and (
        ip := forwarded.split(",")[0].strip()
    ):
        return ip
    if request.client:
        return request.client.host
    return "unknown"


app = FastAPI(
    title="deploy-baba-agent",
    description="PydanticAI agentic service for sislam.com",
    version="0.2.0",
)
app.include_router(linkedin_router)


def _load_anthropic_key() -> None:
    """Load Anthropic API key from Secrets Manager if not already set."""
    if os.environ.get("ANTHROPIC_API_KEY"):
        return
    arn = os.environ.get("ANTHROPIC_API_KEY_ARN")
    if not arn:
        return
    try:
        import boto3
        import botocore.config

        cfg = botocore.config.Config(connect_timeout=5, read_timeout=5, retries={"max_attempts": 1})
        client = boto3.client(
            "secretsmanager",
            region_name=os.environ.get("AWS_REGION", "us-east-1"),
            config=cfg,
        )
        secret = client.get_secret_value(SecretId=arn)
        raw = secret["SecretString"].strip()
        if raw.startswith("{"):
            data = json.loads(raw)
            key = data.get("anthropic_api_key", "") or data.get("ANTHROPIC_ACCESS_KEY", "")
        else:
            key = raw
        os.environ["ANTHROPIC_API_KEY"] = key
    except Exception as exc:
        logger.warning("Failed to load Anthropic key from Secrets Manager: %s", exc)


def _trim_resume(resume: dict[str, Any]) -> dict[str, Any]:
    """Keep only fields relevant for cover letter generation."""
    return {
        "name": resume.get("name", ""),
        "title": resume.get("title", ""),
        "bio": resume.get("bio", ""),
        "jobs": resume.get("jobs", []),
        "competencies": resume.get("competencies", []),
    }


async def _build_deps(job_description: str) -> AgentDeps:
    """Pre-ground: fetch resume + match keywords before the agent starts."""
    resume = await fetch_resume()
    bullets = await match_keywords(job_description)

    return AgentDeps(
        resume_summary=json.dumps(_trim_resume(resume), indent=2),
        matched_bullets=bullets,
        job_description=job_description,
        pdf_lambda_name=os.environ.get("PDF_LAMBDA_NAME", ""),
        artifacts_bucket=os.environ.get("ARTIFACTS_BUCKET", ""),
    )


async def _generate_cover_letter_response(
    deps: AgentDeps, job_description: str
) -> CoverLetterResponse:
    agent_timeout = int(os.environ.get("AGENT_TIMEOUT", "120"))
    pdf_timeout = int(os.environ.get("PDF_TIMEOUT", "30"))
    upload_timeout = int(os.environ.get("UPLOAD_TIMEOUT", "20"))

    result = await asyncio.wait_for(
        get_agent().run(
            f"Generate a tailored cover letter draft.\n\nJob Description:\n{job_description}",
            deps=deps,
        ),
        timeout=agent_timeout,
    )
    output = result.output
    pdf_base64 = await asyncio.wait_for(
        convert_html_to_pdf(deps, output.html),
        timeout=pdf_timeout,
    )
    download_url = await asyncio.wait_for(
        upload_pdf_and_link(deps, pdf_base64),
        timeout=upload_timeout,
    )
    return CoverLetterResponse(
        preview_html=output.html,
        download_url=download_url,
        summary=output.summary,
    )


async def _run_cover_letter_job(job_id: str, job_description: str) -> None:
    try:
        await append_event(
            job_id,
            AgentEvent(agent="preground", status="running", detail="Fetching resume data..."),
            "running",
        )
        deps = await _build_deps(job_description)
        await append_event(
            job_id,
            AgentEvent(agent="preground", status="completed", detail="Context loaded"),
            "running",
        )
        await append_event(
            job_id,
            AgentEvent(
                agent="cover_letter_writer", status="running", detail="Generating cover letter..."
            ),
            "running",
        )
        response = await _generate_cover_letter_response(deps, job_description)
        await append_event(
            job_id,
            AgentEvent(
                agent="cover_letter_writer", status="completed", detail="Cover letter generated"
            ),
            "running",
        )
        await append_event(
            job_id,
            AgentEvent(agent="pdf_uploader", status="completed", detail="PDF uploaded to S3"),
            "running",
        )
        await append_event(
            job_id,
            AgentEvent(
                agent="link_generator",
                status="completed",
                detail="Download link ready (valid for 30 days)",
            ),
            "running",
        )
        await complete_job(
            job_id,
            response,
        )
    except TimeoutError:
        agent_timeout = int(os.environ.get("AGENT_TIMEOUT", "120"))
        logger.exception("Cover letter job %s timed out after %s seconds", job_id, agent_timeout)
        await append_event(
            job_id,
            AgentEvent(agent="cover_letter_writer", status="failed", detail="Agent timed out"),
            "failed",
        )
        await fail_job(job_id, f"Agent timed out after {agent_timeout}s")
    except Exception as exc:
        logger.exception("Cover letter job %s failed", job_id)
        await append_event(
            job_id,
            AgentEvent(agent="cover_letter_writer", status="failed", detail=str(exc)[:200]),
            "failed",
        )
        await fail_job(job_id, str(exc))


def _invoke_worker(job_id: str, job_description: str) -> None:
    if os.environ.get("AGENT_JOB_STORE", "memory").lower() != "s3":
        task = asyncio.create_task(_run_cover_letter_job(job_id, job_description))
        _background_tasks.add(task)
        task.add_done_callback(_background_tasks.discard)
        return

    function_name = os.environ.get("AWS_LAMBDA_FUNCTION_NAME")
    if not function_name:
        task = asyncio.create_task(_run_cover_letter_job(job_id, job_description))
        _background_tasks.add(task)
        task.add_done_callback(_background_tasks.discard)
        return

    cfg = botocore.config.Config(connect_timeout=5, read_timeout=10, retries={"max_attempts": 1})
    client = boto3.client(
        "lambda",
        region_name=os.environ.get("AWS_REGION", "us-east-1"),
        config=cfg,
    )
    payload = {
        "source": "deploy-baba.agent",
        "action": "cover_letter_worker",
        "job_id": job_id,
        "job_description": job_description,
    }
    client.invoke(
        FunctionName=function_name,
        InvocationType="Event",
        Payload=json.dumps(payload).encode(),
    )


@app.on_event("startup")
async def startup() -> None:
    _load_anthropic_key()
    if os.environ.get("ANTHROPIC_API_KEY"):
        logger.info(
            "ANTHROPIC_API_KEY is set (starts with %s...)",
            os.environ["ANTHROPIC_API_KEY"][:8],
        )
    else:
        logger.warning("ANTHROPIC_API_KEY is NOT set — agent calls will fail")
    load_linkedin_credentials()
    await _restore_token()


@app.get("/health")
async def health() -> dict[str, str]:
    return {"status": "ok", "service": "agent"}


@app.post("/api/v1/agent/cover-letter", response_model=CoverLetterResponse)
async def cover_letter(request: Request, body: CoverLetterRequest) -> CoverLetterResponse:
    """Generate a tailored cover letter from a job description."""
    if not os.environ.get("ANTHROPIC_API_KEY"):
        raise HTTPException(status_code=503, detail="ANTHROPIC_API_KEY not configured")
    ip = _extract_client_ip(request)
    if not _check_rate_limit(ip):
        raise HTTPException(
            status_code=429,
            detail=f"Rate limit exceeded ({AGENT_RATE_LIMIT}/day)",
        )
    try:
        deps = await _build_deps(body.job_description)
        response = await _generate_cover_letter_response(deps, body.job_description)
    except TimeoutError as err:
        agent_timeout = int(os.environ.get("AGENT_TIMEOUT", "120"))
        raise HTTPException(
            status_code=504,
            detail=f"Agent timed out after {agent_timeout}s — check ANTHROPIC_API_KEY is set",
        ) from err

    if not response.download_url:
        raise HTTPException(
            status_code=500, detail="Cover letter generation failed — no download URL"
        )

    return response


@app.post("/api/v1/agent/cover-letter/jobs", response_model=CoverLetterJobCreateResponse)
async def create_cover_letter_job(
    request: Request, body: CoverLetterRequest
) -> CoverLetterJobCreateResponse:
    if not os.environ.get("ANTHROPIC_API_KEY"):
        raise HTTPException(status_code=503, detail="ANTHROPIC_API_KEY not configured")
    ip = _extract_client_ip(request)
    if not _check_rate_limit(ip):
        raise HTTPException(
            status_code=429,
            detail=f"Rate limit exceeded ({AGENT_RATE_LIMIT}/day)",
        )
    job_id = uuid.uuid4().hex
    job = await create_job(job_id)
    _invoke_worker(job_id, body.job_description)
    return CoverLetterJobCreateResponse(job_id=job.job_id, status=job.status)


@app.get("/api/v1/agent/cover-letter/jobs/{job_id}", response_model=CoverLetterJobStatus)
async def get_cover_letter_job(job_id: str) -> CoverLetterJobStatus:
    job = await get_job(job_id)
    if job is None:
        raise HTTPException(status_code=404, detail="Cover letter job not found")
    return job


@app.post("/api/v1/agent/cover-letter/stream", response_model=None)
async def cover_letter_stream(
    request: Request, body: CoverLetterRequest
) -> StreamingResponse | JSONResponse:
    """Stream cover letter generation with real-time agent status updates."""
    if not os.environ.get("ANTHROPIC_API_KEY"):
        return JSONResponse(
            status_code=503,
            content={"detail": "ANTHROPIC_API_KEY not configured"},
        )
    ip = _extract_client_ip(request)
    if not _check_rate_limit(ip):
        return JSONResponse(
            status_code=429,
            content={"detail": f"Rate limit exceeded ({AGENT_RATE_LIMIT}/day)"},
        )

    async def event_generator() -> AsyncGenerator[str]:
        try:
            # Pre-grounding phase
            event = AgentEvent(
                agent="preground", status="running", detail="Fetching resume data..."
            )
            yield f"event: agent\ndata: {event.model_dump_json()}\n\n"

            deps = await _build_deps(body.job_description)

            event = AgentEvent(agent="preground", status="completed", detail="Context loaded")
            yield f"event: agent\ndata: {event.model_dump_json()}\n\n"

            # Agent execution phase
            event = AgentEvent(
                agent="cover_letter_writer", status="running", detail="Generating cover letter..."
            )
            yield f"event: agent\ndata: {event.model_dump_json()}\n\n"

            response = await _generate_cover_letter_response(deps, body.job_description)

            event = AgentEvent(
                agent="cover_letter_writer", status="completed", detail="Cover letter generated"
            )
            yield f"event: agent\ndata: {event.model_dump_json()}\n\n"

            event = AgentEvent(
                agent="pdf_uploader", status="completed", detail="PDF uploaded to S3"
            )
            yield f"event: agent\ndata: {event.model_dump_json()}\n\n"

            event = AgentEvent(
                agent="link_generator",
                status="completed",
                detail="Download link ready (valid for 30 days)",
            )
            yield f"event: agent\ndata: {event.model_dump_json()}\n\n"

            # Final result
            final_result: dict[str, Any] = {
                "download_url": response.download_url,
                "preview_html": response.preview_html,
                "summary": response.summary,
            }
            yield f"event: result\ndata: {json.dumps(final_result)}\n\n"
            yield "event: done\ndata: {}\n\n"

        except TimeoutError:
            logger.error(
                "Agent timed out after %s seconds",
                os.environ.get("AGENT_TIMEOUT", "120"),
            )
            event = AgentEvent(
                agent="cover_letter_writer", status="failed", detail="Agent timed out"
            )
            yield f"event: agent\ndata: {event.model_dump_json()}\n\n"
            msg = json.dumps({"message": "Agent timed out — check ANTHROPIC_API_KEY is set"})
            yield f"event: error\ndata: {msg}\n\n"
        except Exception as exc:
            logger.exception("Agent stream error")
            event = AgentEvent(agent="cover_letter_writer", status="failed", detail=str(exc)[:200])
            yield f"event: agent\ndata: {event.model_dump_json()}\n\n"
            error_event = {"message": str(exc)}
            yield f"event: error\ndata: {json.dumps(error_event)}\n\n"

    return StreamingResponse(
        event_generator(),
        media_type="text/event-stream",
        headers={"Cache-Control": "no-cache", "Connection": "keep-alive"},
    )


handler = Mangum(app, lifespan="auto")


def lambda_handler(event: dict[str, Any], context: Any) -> Any:
    if event.get("source") == "deploy-baba.agent" and event.get("action") == "cover_letter_worker":
        _load_anthropic_key()
        asyncio.run(_run_cover_letter_job(event["job_id"], event["job_description"]))
        return {"statusCode": 202, "body": ""}
    return handler(event, context)


if __name__ == "__main__":
    import uvicorn

    uvicorn.run("handler:app", host="0.0.0.0", port=3003, reload=True)
