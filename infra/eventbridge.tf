# EventBridge rule for scheduled database backups
resource "aws_cloudwatch_event_rule" "backup_schedule" {
  name                = "${local.lambda_function_name}-backup-schedule"
  description         = "Trigger database backups on a schedule"
  schedule_expression = var.backup_schedule
  is_enabled          = true

  tags = {
    Name = "${local.lambda_function_name}-backup-schedule"
  }
}

# EventBridge target - invoke the main Lambda with backup event
resource "aws_cloudwatch_event_target" "backup_lambda" {
  rule      = aws_cloudwatch_event_rule.backup_schedule.name
  target_id = "BackupLambda"
  arn       = aws_lambda_function.baba.arn

  input = jsonencode({
    action = "backup"
    source = "eventbridge"
  })
}

# Lambda permission for EventBridge to invoke
resource "aws_lambda_permission" "eventbridge_invoke" {
  statement_id  = "AllowEventBridgeInvoke"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.baba.function_name
  principal     = "events.amazonaws.com"
  source_arn    = aws_cloudwatch_event_rule.backup_schedule.arn
}
