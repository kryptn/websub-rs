
module "subscribe_function" {
  source  = "./lambda"
  name    = "websub-subscribe"
  archive = "../target/lambda/websub-subscribe/bootstrap.zip"
}

