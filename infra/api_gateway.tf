
resource "aws_api_gateway_rest_api" "websub" {
  name = "websub"
}

resource "aws_api_gateway_resource" "proxy" {
  rest_api_id = aws_api_gateway_rest_api.websub.id
  parent_id   = aws_api_gateway_rest_api.websub.root_resource_id
  path_part   = "{proxy+}"
}

resource "aws_api_gateway_method" "proxy_post" {
  rest_api_id   = aws_api_gateway_rest_api.websub.id
  resource_id   = aws_api_gateway_resource.proxy.id
  http_method   = "POST"
  authorization = "NONE"
}

resource "aws_api_gateway_method" "proxy_get" {
  rest_api_id   = aws_api_gateway_rest_api.websub.id
  resource_id   = aws_api_gateway_resource.proxy.id
  http_method   = "GET"
  authorization = "NONE"
}

resource "aws_api_gateway_deployment" "websub-deploy" {
  depends_on = [
    aws_api_gateway_integration.webhook,
    aws_api_gateway_integration.challenge,
  ]

  rest_api_id = aws_api_gateway_rest_api.websub.id
  stage_name  = "test"
}

output "api_gateway_url" {
  value = aws_api_gateway_deployment.websub-deploy.invoke_url
}