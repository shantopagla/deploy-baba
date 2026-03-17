# CloudWatch Log Group for Lambda
resource "aws_cloudwatch_log_group" "lambda" {
  name              = "/aws/lambda/${local.lambda_function_name}"
  retention_in_days = var.logs_retention_days

  tags = {
    Name = "${local.lambda_function_name}-logs"
  }
}

# Lambda function for the Baba portfolio site
resource "aws_lambda_function" "baba" {
  filename            = var.lambda_code_path
  function_name       = local.lambda_function_name
  role                = aws_iam_role.lambda_execution.arn
  handler             = "index.handler"
  runtime             = "provided.al2023"
  memory_size         = var.lambda_memory
  timeout             = var.lambda_timeout
  architectures       = ["arm64"]
  source_code_hash    = filebase64sha256(var.lambda_code_path)

  # Environment variables passed to the Lambda function
  environment {
    variables = {
      DB_PATH = "/mnt/db/baba.db"
      RUST_LOG = "info"
    }
  }

  # EFS mount configuration
  file_system_config {
    arn              = aws_efs_access_point.db.arn
    local_mount_path = "/mnt/db"
  }

  # VPC configuration for EFS access
  vpc_config {
    subnet_ids         = data.aws_subnets.default.ids
    security_group_ids = [aws_security_group.lambda_efs.id]
  }

  # Explicit CloudWatch Logs dependency
  depends_on = [
    aws_cloudwatch_log_group.lambda,
    aws_iam_role_policy_attachment.lambda_logs,
    aws_iam_role_policy_attachment.lambda_efs,
    aws_iam_role_policy_attachment.lambda_s3,
    aws_iam_role_policy_attachment.lambda_ssm,
  ]

  tags = {
    Name = local.lambda_function_name
  }
}

# Lambda Function URL for direct HTTPS invocation (no API Gateway)
resource "aws_lambda_function_url" "baba" {
  function_name          = aws_lambda_function.baba.function_name
  authorization_type    = "NONE"
  cors {
    allow_origins = ["*"]
    allow_methods = ["GET", "POST", "OPTIONS"]
    allow_headers = ["Content-Type"]
    expose_headers = ["Content-Type"]
    max_age       = 3600
  }

  depends_on = [aws_lambda_function.baba]
}

# Lambda permission for invocation via Function URL
resource "aws_lambda_permission" "function_url" {
  statement_id       = "AllowPublicInvoke"
  action             = "lambda:InvokeFunctionUrl"
  function_name      = aws_lambda_function.baba.function_name
  principal          = "*"
}
