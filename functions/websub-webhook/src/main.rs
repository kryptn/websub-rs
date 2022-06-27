use std::env;

use feed_rs::parser;
use lambda_http::{run, service_fn, Body, Error, IntoResponse, Request, RequestExt, Response};
use tracing_subscriber::EnvFilter;
use uuid::Uuid;
use websub::{Message, WebsubClient};

async fn function_handler(event: Request) -> Result<impl IntoResponse, Error> {
    let path_params = event.path_parameters();
    let subscription_id = path_params
        .first("subscription_id")
        .expect("we are providing this value");
    let subscription_id = Uuid::parse_str(subscription_id)?;

    let feed = if let Body::Text(body) = event.body() {
        tracing::info!("message body: {}", body);
        parser::parse(body.as_bytes())?
    } else {
        panic!("expected a text body");
    };

    dbg!(&feed);

    let entry = feed.entries.first().unwrap();

    let author = entry
        .authors
        .first()
        .map(|a| a.name.clone())
        .unwrap_or_else(|| "unknown author".to_string());
    let link = entry.links.first().unwrap();

    let websub = WebsubClient::default().await;

    let consumers = websub
        .get_handlers_for_subscription(subscription_id)
        .await?;

    let msg_body = format!("{} -- {}", author, link.href);
    for consumer in consumers {
        let message = Message::new(
            entry.id.clone(),
            consumer.consumer_name,
            Some(subscription_id),
            msg_body.clone(),
        );

        websub.put_message(&message).await?;
    }

    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(())
        .map_err(Box::new)?;
    Ok(resp)
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
