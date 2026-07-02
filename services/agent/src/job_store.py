from __future__ import annotations

import asyncio
import json
import os
from typing import Any, Literal

import boto3
import botocore.config
from botocore.exceptions import ClientError

from models import AgentEvent, CoverLetterJobStatus, CoverLetterResponse

JobStatus = Literal["pending", "running", "completed", "failed"]

_memory_jobs: dict[str, CoverLetterJobStatus] = {}
_memory_lock = asyncio.Lock()


def _store_kind() -> str:
    return os.environ.get("AGENT_JOB_STORE", "memory").lower()


def _bucket() -> str:
    return os.environ["ARTIFACTS_BUCKET"]


def _prefix() -> str:
    prefix = os.environ.get("AGENT_JOBS_PREFIX", "agent-jobs/")
    return prefix if prefix.endswith("/") else f"{prefix}/"


def _key(job_id: str) -> str:
    return f"{_prefix()}{job_id}.json"


def _s3_client() -> Any:
    cfg = botocore.config.Config(connect_timeout=5, read_timeout=10, retries={"max_attempts": 2})
    endpoint_url = os.environ.get("S3_ENDPOINT_URL")
    return boto3.client(
        "s3",
        region_name=os.environ.get("AWS_REGION", "us-east-1"),
        config=cfg,
        **({"endpoint_url": endpoint_url} if endpoint_url else {}),
    )


async def create_job(job_id: str) -> CoverLetterJobStatus:
    job = CoverLetterJobStatus(job_id=job_id, status="pending")
    await save_job(job)
    return job


async def get_job(job_id: str) -> CoverLetterJobStatus | None:
    if _store_kind() == "s3":
        return await asyncio.to_thread(_get_job_s3, job_id)
    async with _memory_lock:
        return _memory_jobs.get(job_id)


async def save_job(job: CoverLetterJobStatus) -> None:
    if _store_kind() == "s3":
        await asyncio.to_thread(_save_job_s3, job)
        return
    async with _memory_lock:
        _memory_jobs[job.job_id] = job


async def append_event(job_id: str, event: AgentEvent, status: JobStatus | None = None) -> None:
    job = await get_job(job_id)
    if job is None:
        job = CoverLetterJobStatus(job_id=job_id, status=status or "running")
    if status is not None:
        job.status = status
    job.events.append(event)
    await save_job(job)


async def complete_job(job_id: str, result: CoverLetterResponse) -> None:
    job = await get_job(job_id) or CoverLetterJobStatus(job_id=job_id, status="completed")
    job.status = "completed"
    job.result = result
    job.error = None
    await save_job(job)


async def fail_job(job_id: str, message: str) -> None:
    job = await get_job(job_id) or CoverLetterJobStatus(job_id=job_id, status="failed")
    job.status = "failed"
    job.error = message
    await save_job(job)


def _get_job_s3(job_id: str) -> CoverLetterJobStatus | None:
    try:
        response = _s3_client().get_object(Bucket=_bucket(), Key=_key(job_id))
    except ClientError as exc:
        if exc.response.get("Error", {}).get("Code") in {"NoSuchKey", "404"}:
            return None
        raise
    data: dict[str, Any] = json.loads(response["Body"].read())
    return CoverLetterJobStatus.model_validate(data)


def _save_job_s3(job: CoverLetterJobStatus) -> None:
    body = job.model_dump_json().encode()
    _s3_client().put_object(
        Bucket=_bucket(),
        Key=_key(job.job_id),
        Body=body,
        ContentType="application/json",
    )
