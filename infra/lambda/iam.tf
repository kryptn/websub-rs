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

  managed_policy_arns = [ aws_iam_policy.ssm_read.arn]
}

resource "aws_iam_policy" "ssm_read" {
    name = "${var.name}-ssm-read"
    policy = jsonencode({
            Version = "2012-10-17"
    Statement = [
      {
        Action   =[
              "ssm:GetParameter",
              "ssm:GetParameters",
              "ssm:GetParametersByPath",
            ]
        Effect   = "Allow"
        Resource = "arn:aws:ssm:::parameter/websub/"
      },
    ]
    })

}

resource "aws_iam_role_policy_attachment" "basic_policy" {
  role       = aws_iam_role.exec.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
}

resource "aws_iam_role_policy_attachment" "dynamodb_policy" {
  role       = aws_iam_role.exec.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaDynamoDBExecutionRole"
}

