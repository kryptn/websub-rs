resource "aws_dynamodb_table" "subscriptions" {
  name         = "subscriptions"
  billing_mode = "PAY_PER_REQUEST"
  hash_key     = "Id"
  range_key    = ""

  attribute {
    name = "Id"
    type = "S"
  }

  stream_enabled   = true
  stream_view_type = "NEW_AND_OLD_IMAGES"
}

resource "aws_dynamodb_table" "subscription_leases" {
  name         = "subscription_leases"
  billing_mode = "PAY_PER_REQUEST"
  hash_key     = "SubscriptionId"

  attribute {
    name = "SubscriptionId"
    type = "S"
  }

  ttl {
    enabled        = true
    attribute_name = "Expiry"
  }

  stream_enabled   = true
  stream_view_type = "NEW_AND_OLD_IMAGES"
}

resource "aws_dynamodb_table" "subscription_handlers" {
  name         = "subscription_handlers"
  billing_mode = "PAY_PER_REQUEST"
  hash_key     = "SubscriptionId"
  range_key    = "Handler"

  attribute {
    name = "SubscriptionId"
    type = "S"
  }

  attribute {
    name = "Handler"
    type = "S"
  }

  local_secondary_index {
    name            = "handler_idx"
    projection_type = "KEYS_ONLY"
    range_key       = "Handler"
  }

  # attribute {
  #     name = "Expiry"
  #     type = "N"
  # }

  # ttl {
  #     enabled = true
  #     attribute_name = "Expiry"
  # }

  # stream_enabled = true
  # stream_view_type = "NEW_AND_OLD_IMAGES"
}


resource "aws_dynamodb_table" "messages" {
  name         = "callbacks"
  billing_mode = "PAY_PER_REQUEST"
  hash_key     = "Id"
  range_key    = "SubscriptionId"

  attribute {
    name = "Id"
    type = "S"
  }

  attribute {
    name = "SubscriptionId"
    type = "S"
  }


  ttl {
    enabled        = true
    attribute_name = "Expiry"
  }

  stream_enabled   = true
  stream_view_type = "NEW_AND_OLD_IMAGES"
}