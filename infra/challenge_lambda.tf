
module "challenge_function" {
  source  = "./lambda"
  name    = "websub-challenge-response"
  archive = "../target/lambda/websub-challenge-response/bootstrap.zip"

  region = var.region

  environment = {
    VERIFY_TOKEN_PARAM = var.websub_verify_token_param
  }
}

resource "aws_api_gateway_integration" "challenge" {
  rest_api_id = aws_api_gateway_rest_api.websub.id
  resource_id = aws_api_gateway_method.proxy_get.resource_id
  http_method = aws_api_gateway_method.proxy_get.http_method

  integration_http_method = "POST"
  type                    = "AWS_PROXY"
  uri                     = module.challenge_function.lambda_invoke_arn
}



resource "aws_lambda_permission" "challenge" {
  statement_id  = "AllowAPIGatewayInvoke"
  action        = "lambda:InvokeFunction"
  function_name = module.challenge_function.lambda_name
  principal     = "apigateway.amazonaws.com"

  source_arn = "${aws_api_gateway_rest_api.websub.execution_arn}/*/*"
}


output "challenge_lambda_arn" {
  value = module.challenge_function.lambda_arn
}

output "challenge_lambda_invoke_arn" {
  value = module.challenge_function.lambda_invoke_arn
}

output "challenge_lambda_name" {
  value = module.challenge_function.lambda_name
}