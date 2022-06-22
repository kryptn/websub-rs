resource "aws_iam_role" "exec" {
  name = "${var.name}-exec"

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

# resource "aws_iam_policy" "ssm_read" {
#     name = "${var.name}-ssm-read"
#     policy = jsonencode({
#             Version = "2012-10-17"
#     Statement = [
#       {
#         Action   =[
#               "ssm:GetParameter",
#               "ssm:GetParameters",
#               "ssm:GetParametersByPath",
#             ]
#         Effect   = "Allow"
#         Resource = "arn:aws:ssm:::parameter/websub/"
#       },
#     ]
#     })

# }


data "aws_iam_policy_document" "ssm_read_policy" {
  statement {
    actions = [
      "ssm:GetParameter",
      "ssm:GetParameters",
      "ssm:GetParametersByPath"
    ]
    effect = "Allow"
    resources = ["arn:aws:ssm:::parameter/websub", "arn:aws:ssm:::parameter/api_gateway/websub/"]
  }
}

resource "aws_iam_policy" "ssm_policy" {
  policy = data.aws_iam_policy_document.ssm_read_policy.json
}

resource "aws_iam_role_policy_attachment" "ssm_policy" {
    role       = aws_iam_role.exec.name
  policy_arn = aws_iam_policy.ssm_policy.arn
}


data "aws_iam_policy" "basic_policy" {
  arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
}

resource "aws_iam_role_policy_attachment" "basic_policy" {
  role       = aws_iam_role.exec.name
  policy_arn = data.aws_iam_policy.basic_policy.arn
}

data "aws_iam_policy_document" "dynamodb_policy" {
  statement {
    actions = [
                "dynamodb:DescribeStream",
                "dynamodb:GetRecords",
                "dynamodb:GetShardIterator",
                "dynamodb:ListStreams",
                "dynamodb:GetItem",
                "dynamodb:PutItem",
                "dynamodb:DeleteItem",
                "dynamodb:Query",
                "logs:CreateLogGroup",
                "logs:CreateLogStream",
                "logs:PutLogEvents"
    ]
    effect = "Allow"
    resources = ["*"]
  }
}

resource "aws_iam_policy" "dynamodb_policy" {
  policy = data.aws_iam_policy_document.dynamodb_policy.json
}

resource "aws_iam_role_policy_attachment" "dynamodb_policy" {
  role       = aws_iam_role.exec.name
  policy_arn = aws_iam_policy.dynamodb_policy.arn
}

