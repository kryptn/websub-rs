
module "notify_function" {
  source  = "./lambda"
  name    = "websub-notify"
  archive = "../target/lambda/websub-notify/bootstrap.zip"

  region = var.region
}

resource "aws_lambda_event_source_mapping" "message_added" {
  event_source_arn  = aws_dynamodb_table.messages.stream_arn
  function_name     = module.notify_function.lambda_arn
  starting_position = "LATEST"

  filter_criteria {
    filter {
      pattern = jsonencode({
        eventName : ["INSERT"]
      })
    }
  }
}


output "notify_lambda_arn" {
  value = module.notify_function.lambda_arn
}

output "notify_lambda_invoke_arn" {
  value = module.notify_function.lambda_invoke_arn
}

output "notify_lambda_name" {
  value = module.notify_function.lambda_name
}
