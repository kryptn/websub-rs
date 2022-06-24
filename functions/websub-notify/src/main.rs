use std::env;

use aws_lambda_events::event::dynamodb::Event;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use serde_dynamo::from_item;
use tracing_subscriber::EnvFilter;
use websub::{Message, WebsubClient};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SlackMessage {
    text: String,
}

impl From<Message> for SlackMessage {
    fn from(msg: Message) -> Self {
        Self { text: msg.body }
    }
}

async fn notify(
    client: reqwest::Client,
    websub: WebsubClient,
    message: Message,
) -> Result<(), Error> {
    let consumer = websub
        .get_consumer(&message.consumer_name)
        .await?
        .expect("lazy");
    let payload: SlackMessage = message.into();

    client
        .post(consumer.slack_url().unwrap())
        .json(&payload)
        .send()
        .await?
        .error_for_status()?;
    //client.post(message.)

    Ok(())
}

async fn function_handler(event: LambdaEvent<Event>) -> Result<(), Error> {
    let webusb_client = WebsubClient::default().await;
    let client = reqwest::ClientBuilder::new().use_rustls_tls().build()?;

    for record in event.payload.records {
        let item: Message = from_item(record.change.new_image)?;
        notify(client.clone(), webusb_client.clone(), item.into()).await?;
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
