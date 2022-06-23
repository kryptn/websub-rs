variable "region" {
  default = "us-west-2"
  type    = string
}

variable "api_gateway_url_param" {
  default = "/api_gateway/websub/invokeUrl"
  type    = string
}

variable "websub_verify_token_param" {
  default = "/lambda/websub/verify_token"
  type    = string
}