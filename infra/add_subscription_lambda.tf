
module "add_subscription_function" {
  source  = "./lambda"
  name    = "websub-add-subscription"
  archive = "../target/lambda/websub-add-subscription/bootstrap.zip"

  region = var.region
}


output "add_subscription_lambda_arn" {
  value = module.add_subscription_function.lambda_arn
}

output "add_subscription_lambda_invoke_arn" {
  value = module.add_subscription_function.lambda_invoke_arn
}

output "add_subscription_lambda_name" {
  value = module.add_subscription_function.lambda_name
}
