resource "aws_dynamodb_table" "subscriptions" {
  name         = "subscriptions"
  billing_mode = "PAY_PER_REQUEST"
  hash_key     = "id"
  range_key    = ""

  attribute {
    name = "id"
    type = "S"
  }

  stream_enabled   = true
  stream_view_type = "NEW_AND_OLD_IMAGES"
}

resource "aws_dynamodb_table" "subscription_leases" {
  name         = "subscription_leases"
  billing_mode = "PAY_PER_REQUEST"
  hash_key     = "subscription_id"

  attribute {
    name = "subscription_id"
    type = "S"
  }

  ttl {
    enabled        = true
    attribute_name = "expiry"
  }

  stream_enabled   = true
  stream_view_type = "NEW_AND_OLD_IMAGES"
}

resource "aws_dynamodb_table" "subscription_handlers" {
  name         = "subscription_handlers"
  billing_mode = "PAY_PER_REQUEST"
  hash_key     = "subscription_id"
  range_key    = "handler_id"

  attribute {
    name = "subscription_id"
    type = "S"
  }

  attribute {
    name = "handler_id"
    type = "S"
  }

  local_secondary_index {
    name            = "handler_idx"
    projection_type = "KEYS_ONLY"
    range_key       = "handler_id"
  }

  # attribute {
  #     name = "expiry"
  #     type = "N"
  # }

  # ttl {
  #     enabled = true
  #     attribute_name = "expiry"
  # }

  # stream_enabled = true
  # stream_view_type = "NEW_AND_OLD_IMAGES"
}


resource "aws_dynamodb_table" "messages" {
  name         = "messages"
  billing_mode = "PAY_PER_REQUEST"
  hash_key     = "id"
  range_key    = "consumer_name"

  attribute {
    name = "id"
    type = "S"
  }

  attribute {
    name = "consumer_name"
    type = "S"
  }


  ttl {
    enabled        = true
    attribute_name = "expiry"
  }

  stream_enabled   = true
  stream_view_type = "NEW_AND_OLD_IMAGES"
}

resource "aws_dynamodb_table" "consumers" {
  name         = "consumers"
  billing_mode = "PAY_PER_REQUEST"
  hash_key     = "name"


  attribute {
    name = "name"
    type = "S"
  }

  stream_enabled   = true
  stream_view_type = "NEW_AND_OLD_IMAGES"
}