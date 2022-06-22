use std::env;

use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use tracing_subscriber::EnvFilter;
use websub::{AddSubscription, Subscription, WebsubClient};

async fn function_handler(event: LambdaEvent<AddSubscription>) -> Result<Subscription, Error> {
    let client = WebsubClient::default().await;

    let subscription = event.payload.into();
    client.create_subscription(&subscription).await?;

    Ok(subscription)
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
