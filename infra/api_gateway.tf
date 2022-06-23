
resource "aws_api_gateway_rest_api" "websub" {
  name = "websub"

}

resource "aws_api_gateway_resource" "proxy" {
  rest_api_id = aws_api_gateway_rest_api.websub.id
  parent_id   = aws_api_gateway_rest_api.websub.root_resource_id
  path_part   = "{subscription_id}"
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
}


resource "aws_cloudwatch_log_group" "gateway_logs" {
  name              = "API-Gateway-Execution-Logs_${aws_api_gateway_rest_api.websub.id}/test"
  retention_in_days = 7

}

resource "aws_api_gateway_stage" "test" {
  depends_on = [aws_cloudwatch_log_group.gateway_logs]

  rest_api_id   = aws_api_gateway_rest_api.websub.id
  deployment_id = aws_api_gateway_deployment.websub-deploy.id
  stage_name    = "test"
}

output "api_gateway_url" {
  value = aws_api_gateway_deployment.websub-deploy.invoke_url
}

data "aws_iam_policy_document" "api_gateway_logs" {
  statement {
    actions = [
      "logs:CreateLogGroup",
      "logs:CreateLogStream",
      "logs:DescribeLogGroups",
      "logs:DescribeLogStreams",
      "logs:PutLogEvents",
      "logs:GetLogEvents",
      "logs:FilterLogEvents"

    ]
    effect    = "Allow"
    resources = ["*"]
  }
}

resource "aws_iam_role_policy" "api_gateway_logs" {
  name   = "websub-api-gateway-logs"
  policy = data.aws_iam_policy_document.api_gateway_logs.json
  role   = aws_iam_role.api_gateway_cloudwatch.id
}

data "aws_iam_policy_document" "api_gateway_assume_role" {
  statement {
    effect  = "Allow"
    actions = ["sts:AssumeRole"]
    principals {
      type        = "Service"
      identifiers = ["apigateway.amazonaws.com"]
    }
  }
}

resource "aws_iam_role" "api_gateway_cloudwatch" {
  name               = "api_gateway_cloudwatch"
  assume_role_policy = data.aws_iam_policy_document.api_gateway_assume_role.json
}

resource "aws_api_gateway_account" "apigateway" {
  cloudwatch_role_arn = aws_iam_role.api_gateway_cloudwatch.arn
}