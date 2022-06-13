
module "challenge_function" {
  source  = "./lambda"
  name    = "websub-challenge-response"
  archive = "../target/lambda/websub-challenge-response/bootstrap.zip"
}

resource "aws_api_gateway_integration" "challenge" {
  rest_api_id = aws_api_gateway_rest_api.websub.id
  resource_id = aws_api_gateway_method.proxy_get.resource_id
  http_method = aws_api_gateway_method.proxy_get.http_method

  integration_http_method = "GET"
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