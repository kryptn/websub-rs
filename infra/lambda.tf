
locals {
  lambda_archive = "../target/lambda/websub-lambda/bootstrap.zip"
}

resource "aws_lambda_function" "test_lambda" {
  # If the file is not in the current working directory you will need to include a
  # path.module in the filename.
  filename      = local.lambda_archive
  function_name = "websub-lambda"
  role          = aws_iam_role.lambda_exec.arn
  handler       = "bootstrap"

  # The filebase64sha256() function is available in Terraform 0.11.12 and later
  # For Terraform 0.11.11 and earlier, use the base64sha256() function and the file() function:
  # source_code_hash = "${base64sha256(file("lambda_function_payload.zip"))}"
  source_code_hash = filebase64sha256(local.lambda_archive)

  runtime = "provided.al2"

  environment {
    variables = {
      RUST_LOG = "trace"
    }
  }
}

resource "aws_cloudwatch_log_group" "test_lambda" {
  name = "/aws/lambda/${aws_lambda_function.test_lambda.function_name}"

  retention_in_days = 30
}

output "invoke_arn" {
  value = aws_lambda_function.test_lambda.invoke_arn
}