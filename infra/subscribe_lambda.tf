
module "subscribe_function" {
  source  = "./lambda"
  name    = "websub-subscribe"
  archive = "../target/lambda/websub-subscribe/bootstrap.zip"

  region = var.region
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