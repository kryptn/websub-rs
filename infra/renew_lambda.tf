
module "renew_function" {
  source  = "./lambda"
  name    = "websub-renew"
  archive = "../target/lambda/websub-renew/bootstrap.zip"

  region = var.region
}

resource "aws_lambda_event_source_mapping" "lease_expired" {
  event_source_arn  = aws_dynamodb_table.subscription_leases.stream_arn
  function_name     = module.renew_function.lambda_arn
  starting_position = "LATEST"

  filter_criteria {
    filter {
      pattern = jsonencode({
        userIdentity : {
          type : "Service",
          principalId : "dynamodb.amazonaws.com",
        }
      })
    }
  }
}

output "renew_lambda_arn" {
  value = module.renew_function.lambda_arn
}

output "renew_lambda_invoke_arn" {
  value = module.renew_function.lambda_invoke_arn
}

output "renew_lambda_name" {
  value = module.renew_function.lambda_name
}