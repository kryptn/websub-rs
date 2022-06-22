use std::env;

use aws_lambda_events::event::dynamodb::Event;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde_dynamo::from_item;
use tracing_subscriber::EnvFilter;
use websub::Subscription;

async fn handle(subscription: Subscription, client: reqwest::Client) -> Result<(), Error> {
    client
        .post(subscription.hub_url)
        .query(&[
            ("hub.mode", "subscribe"),
            ("hub.topic", &subscription.topic_url),
            ("hub.callback", ""),
            ("hub.verify", "sync"),
            ("hub.verify_token", "password"),
            ("hub.lease_seconds", "3600"),
            ("websub.subscriptionId", &subscription.id.to_string()),
        ])
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/lambda-runtime/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler(event: LambdaEvent<Event>) -> Result<(), Error> {
    // Extract some useful information from the request

    let client = reqwest::Client::new();

    for record in event.payload.records {
        let item: Subscription = from_item(record.change.new_image)?;
        handle(item, client.clone()).await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let builder = tracing_subscriber::fmt()
        .json()
        .with_env_filter(EnvFilter::from_default_env())
        .without_time();

    if env::var("AWS_EXECUTION_ENV").is_ok() {
        builder.json().init();
    } else {
        builder.init();
    }

    run(service_fn(function_handler)).await
}
