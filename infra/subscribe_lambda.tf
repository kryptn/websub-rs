
module "subscribe_function" {
  source  = "./lambda"
  name    = "websub-subscribe"
  archive = "../target/lambda/websub-subscribe/bootstrap.zip"

  region = var.region

  environment = {
    INVOKE_URL_SSM_PARAM = var.api_gateway_url_param
    VERIFY_TOKEN_PARAM   = var.websub_verify_token_param
  }
}

resource "aws_lambda_event_source_mapping" "subscription_added" {
  event_source_arn  = aws_dynamodb_table.subscriptions.stream_arn
  function_name     = module.subscribe_function.lambda_arn
  starting_position = "LATEST"

  filter_criteria {
    filter {
      pattern = jsonencode({
        eventName : ["INSERT", "MODIFY"]
      })
    }
  }
}



output "subscribe_lambda_arn" {
  value = module.subscribe_function.lambda_arn
}

output "subscribe_lambda_invoke_arn" {
  value = module.subscribe_function.lambda_invoke_arn
}

output "subscribe_lambda_name" {
  value = module.subscribe_function.lambda_name
}