






output "lambda_arn" {
  value = aws_lambda_function.fn.arn
}

output "lambda_invoke_arn" {
  value = aws_lambda_function.fn.invoke_arn
}

output "lambda_name" {
  value = aws_lambda_function.fn.function_name
}

output "exec_role_arn" {
  value = aws_iam_role.exec.arn
}

output "exec_role_name" {
  value = aws_iam_role.exec.name
}

