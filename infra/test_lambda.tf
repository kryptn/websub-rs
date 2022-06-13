
locals {
  lambda_name    = "websub-lambda"
  lambda_archive = "../target/lambda/${local.lambda_name}/bootstrap.zip"
}

resource "aws_lambda_function" "test_lambda" {
  # If the file is not in the current working directory you will need to include a
  # path.module in the filename.
  filename      = local.lambda_archive
  function_name = local.lambda_name
  role          = aws_iam_role.test_lambda_exec.arn
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


resource "aws_iam_role" "test_lambda_exec" {
  name = "serverless_lambda"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Action = "sts:AssumeRole"
      Effect = "Allow"
      Sid    = ""
      Principal = {
        Service = "lambda.amazonaws.com"
      }
      }
    ]
  })
}

resource "aws_iam_role_policy_attachment" "test_lambda_policy" {
  role       = aws_iam_role.test_lambda_exec.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
}