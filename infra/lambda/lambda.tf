
variable environment {
  type = map(string)
  default = {}
}

resource "aws_lambda_function" "fn" {
  # If the file is not in the current working directory you will need to include a
  # path.module in the filename.
  filename      = var.archive
  function_name = var.name
  role          = aws_iam_role.exec.arn
  handler       = "bootstrap"

  # The filebase64sha256() function is available in Terraform 0.11.12 and later
  # For Terraform 0.11.11 and earlier, use the base64sha256() function and the file() function:
  # source_code_hash = "${base64sha256(file("lambda_function_payload.zip"))}"
  source_code_hash = filebase64sha256(var.archive)

  runtime = "provided.al2"

  environment {
    variables = merge({
      RUST_LOG = "debug"
    }, var.environment)
  }
}

resource "aws_cloudwatch_log_group" "lambda" {
  name = "/aws/lambda/${aws_lambda_function.fn.function_name}"
  retention_in_days = 30
}