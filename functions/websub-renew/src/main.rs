use std::{env, time::SystemTime};

use aws_lambda_events::event::dynamodb::Event;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde_dynamo::from_item;
use tracing_subscriber::EnvFilter;
use websub::{SubscriptionLease, WebsubClient};

async fn handle(client: WebsubClient, lease: SubscriptionLease) -> Result<(), Error> {
    let subscription = client.get_subscription_by_id(lease.subscription_id).await?;
    if let Some(mut sub) = subscription {
        tracing::info!("renewal candidate found {}", sub.id);
        sub.subscribed_at = {
            let now = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap();
            now.as_secs()
        };
        client.create_subscription(&sub).await?;
        tracing::info!("renewed {}", sub.id);
    }

    Ok(())
}

async fn function_handler(event: LambdaEvent<Event>) -> Result<(), Error> {
    // Extract some useful information from the request

    let client = WebsubClient::default().await;

    for record in event.payload.records {
        let item: SubscriptionLease = from_item(record.change.old_image)?;
        handle(client.clone(), item).await?;
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
