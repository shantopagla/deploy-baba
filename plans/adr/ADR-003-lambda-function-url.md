# ADR-003: Lambda Function URL (No API Gateway)

**Status:** Accepted
**Date:** 2026-03-10
**Affected modules:** W-TF, W-UI

---

## Context

The portfolio site needs an HTTPS endpoint. The common pattern is API Gateway v2
(HTTP API) in front of Lambda. API Gateway adds ~$1/million requests and
introduces an additional resource to manage in Terraform. For a portfolio site
expecting low traffic, this cost and complexity is unnecessary.

AWS introduced Lambda Function URLs in 2022 — a free, built-in HTTPS endpoint
for Lambda functions that requires no API Gateway.

CloudFront sits in front for caching and custom domain (sislam.com).

Alternatives considered:
- **API Gateway v2 (HTTP API)** — $1/M requests, extra TF resource
- **API Gateway v1 (REST API)** — $3.50/M requests, complex
- **Application Load Balancer** — ~$16/month minimum (not free tier)
- **Lambda Function URL** — free, simpler Terraform, direct HTTPS

---

## Decision

Use Lambda Function URL as the origin for CloudFront. No API Gateway.

```
HTTPS Request → CloudFront → Lambda Function URL → Lambda
(sislam.com)   (cache: off)  (origin, HTTPS-only)
```

Configuration:
```hcl
resource "aws_lambda_function_url" "baba_ui" {
  function_name      = aws_lambda_function.baba_ui.function_name
  authorization_type = "NONE"
  cors { ... }
}
```

CloudFront origin points to the Function URL domain.
CloudFront handles: custom domain, ACM SSL cert, HTTPS redirect.

---

## Consequences

**Positive:**
- Lambda Function URL is free (no per-request charge)
- Eliminates API Gateway resource and its IAM permissions from Terraform
- CORS configuration is simpler (single resource)
- Direct invocation possible (bypass CloudFront for debugging)

**Negative:**
- Function URL has no request throttling (CloudFront WAF can mitigate if needed)
- Lambda URL domain format is fixed (`<id>.lambda-url.<region>.on.aws`)
- Custom auth requires `authorization_type = "AWS_IAM"` — not needed for public portfolio

**ECS fallback:** If ECS Fargate Spot is chosen instead (`stack.toml deploy.mode = "ecs-fargate-spot"`),
this ADR doesn't apply — NLB is used instead. See `cross-cutting/aws-architecture.md`.

---

## Cross-References
- `plans/cross-cutting/aws-architecture.md` — full topology diagrams for both options
- `plans/modules/terraform.md` — W-TF Terraform implementation
- `plans/modules/ui-service.md` — W-UI Lambda adapter
