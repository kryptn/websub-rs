
resource "aws_ssm_parameter" "api_gateway_url" {
  name  = var.api_gateway_url_param
  type  = "String"
  value = "${aws_api_gateway_deployment.websub-deploy.invoke_url}${aws_api_gateway_stage.test.stage_name}/"
}

resource "random_password" "verify_token" {
  length  = 36
  special = true
}


resource "aws_ssm_parameter" "websub_verify_token" {
  name  = var.websub_verify_token_param
  type  = "String"
  value = random_password.verify_token.result
}