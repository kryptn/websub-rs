
module "webhook_function" {
  source  = "./lambda"
  name    = "websub-webhook"
  archive = "../target/lambda/websub-webhook/bootstrap.zip"

  region = var.region
}


resource "aws_api_gateway_integration" "webhook" {
  rest_api_id = aws_api_gateway_rest_api.websub.id
  resource_id = aws_api_gateway_method.proxy_post.resource_id
  http_method = aws_api_gateway_method.proxy_post.http_method

  integration_http_method = "POST"
  type                    = "AWS_PROXY"
  uri                     = module.webhook_function.lambda_invoke_arn
}



resource "aws_lambda_permission" "webhook" {
  statement_id  = "AllowAPIGatewayInvoke"
  action        = "lambda:InvokeFunction"
  function_name = module.webhook_function.lambda_name
  principal     = "apigateway.amazonaws.com"

  source_arn = "${aws_api_gateway_rest_api.websub.execution_arn}/*/*"
}



output "webhook_lambda_arn" {
  value = module.webhook_function.lambda_arn
}

output "webhook_lambda_invoke_arn" {
  value = module.webhook_function.lambda_invoke_arn
}

output "webhook_lambda_name" {
  value = module.webhook_function.lambda_name
}