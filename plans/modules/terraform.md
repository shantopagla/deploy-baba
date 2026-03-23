# W-TF: terraform
**Path:** `infra/` | **Status:** DONE (2 deprecation warnings open)
**Depends on:** (external — AWS provider) | **Depended on by:** W-XT (terraform wrapper)

---

## W-TF.1 Purpose

Terraform configuration for all AWS resources. Manages Lambda, EFS, S3 (backup + state),
IAM, CloudWatch, EventBridge, CloudFront (CDN), and SSM. 28 resources total.

→ ADR-003 (Lambda Function URL)
→ ADR-006 (EFS + S3 topology)

---

## W-TF.2 Resource Inventory

| File | Resources |
|------|-----------|
| `main.tf` | Backend config (S3 + DynamoDB), provider |
| `variables.tf` | `lambda_code_path`, `aws_region`, `environment` |
| `outputs.tf` | `function_url`, `cloudfront_domain_name`, `site_url`, `efs_id`, `backup_bucket` |
| `lambda.tf` | `aws_lambda_function`, `aws_lambda_function_url`, `aws_lambda_permission`, `aws_cloudwatch_log_group` |
| `efs.tf` | `aws_efs_file_system`, `aws_efs_access_point`, `aws_efs_mount_target`, SG + SG rules |
| `s3.tf` | Backup bucket, tfstate bucket, versioning, encryption, lifecycle |
| `iam.tf` | Lambda execution role + policies (logs, VPC, EFS, S3, SSM) |
| `ssm.tf` | Sentinel parameter, region, account SSM params |
| `eventbridge.tf` | Scheduled backup rule + target |
| `cdn.tf` | `aws_cloudfront_distribution`, `aws_route53_record` (apex + www) |

---

## W-TF.3 Key Configurations

### Terraform Backend
```hcl
terraform {
  backend "s3" {
    bucket         = "deploy-baba-tfstate"
    key            = "terraform.tfstate"
    region         = "us-east-1"
    dynamodb_table = "terraform-lock"
    encrypt        = true
  }
}
```

### Lambda Function
```hcl
resource "aws_lambda_function" "baba_ui" {
  function_name = "deploy-baba-prod"
  runtime       = "provided.al2023"
  architectures = ["arm64"]
  memory_size   = 256
  timeout       = 30
  filename      = var.lambda_code_path  # infra/build/lambda.zip
  ...
}
```

### EFS Security Groups (no cycle — separate rule resources)
```hcl
# DO NOT use inline ingress/egress when two SGs reference each other
# Instead use aws_security_group_rule resources:
resource "aws_security_group_rule" "efs_ingress_from_lambda" { ... }
resource "aws_security_group_rule" "lambda_egress_to_efs"    { ... }
```
→ DRL-2026-03-18-terraform Entry 2

### Lambda `depends_on` (correct — mix of policy attachment + inline policy types)
```hcl
depends_on = [
  aws_cloudwatch_log_group.lambda,
  aws_iam_role_policy_attachment.lambda_logs,
  aws_iam_role_policy_attachment.lambda_vpc,
  aws_iam_role_policy.lambda_efs,     # inline policy
  aws_iam_role_policy.lambda_s3,      # inline policy
  aws_iam_role_policy.lambda_ssm,     # inline policy
]
```
→ DRL-2026-03-18-terraform Entry 4

---

## W-TF.4 Work Items

| ID | Task | Status | Notes |
|----|------|--------|-------|
| W-TF.4.1 | Fix `eventbridge.tf` deprecation | OPEN | Replace `is_enabled = true` with `state = "ENABLED"` (line 6) |
| W-TF.4.2 | Fix `s3.tf` lifecycle rules | OPEN | Add `filter {}` to each `rule` block in `aws_s3_bucket_lifecycle_configuration` |

---

## W-TF.5 Known Gotchas

1. **Lambda zip prerequisite:** `terraform plan` calls `filebase64sha256(var.lambda_code_path)`
   at plan time. The zip must exist before running `just infra-plan` or `just infra-apply`.
   `just lambda-build` must run first. → DRL-2026-03-18-terraform Entry 5

2. **EFS mount target tags:** `aws_efs_mount_target` does not accept `tags` attribute.
   → DRL-2026-03-18-terraform Entry 3 (already fixed)

3. **CloudFront propagation:** After `just infra-apply`, CloudFront takes 5–15 minutes
   to propagate globally. `just infra-verify DOMAIN` will fail until propagation completes.

---

## W-TF.6 Cross-References
- → ADR-002, ADR-003, ADR-005, ADR-006
- → W-XT (terraform.rs + bootstrap.rs wrapper)
- → `plans/drift/DRL-2026-03-18-terraform.md` — all drift entries
- → `plans/cross-cutting/aws-architecture.md` — topology diagrams
- → `plans/cross-cutting/aws-setup-spec.md` — IAM permissions required
