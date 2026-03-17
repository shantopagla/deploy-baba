# SSM Parameter for database path
resource "aws_ssm_parameter" "db_path" {
  name        = "/${var.project_name}/${var.environment}/db-path"
  description = "Path to SQLite database in EFS"
  type        = "String"
  value       = "/mnt/db/baba.db"

  tags = {
    Name = "${local.lambda_function_name}-db-path"
  }
}

# SSM Parameter for backup bucket name
resource "aws_ssm_parameter" "backup_bucket" {
  name        = "/${var.project_name}/${var.environment}/backup-bucket"
  description = "S3 bucket for database backups"
  type        = "String"
  value       = aws_s3_bucket.backups.id

  tags = {
    Name = "${local.lambda_function_name}-backup-bucket"
  }
}

# SSM Parameter for backup prefix
resource "aws_ssm_parameter" "backup_prefix" {
  name        = "/${var.project_name}/${var.environment}/backup-prefix"
  description = "S3 prefix for database backups"
  type        = "String"
  value       = "backups/"

  tags = {
    Name = "${local.lambda_function_name}-backup-prefix"
  }
}
