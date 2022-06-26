use std::{collections::HashMap, env};

use aws_lambda_events::event::dynamodb::Event;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde_dynamo::from_item;
use tracing_subscriber::EnvFilter;
use websub::{ssm::get_parameters, Subscription};

async fn handle(
    subscription: Subscription,
    base_invoke_url: &str,
    verify_token: &str,
    client: reqwest::Client,
) -> Result<(), Error> {
    // ssm stored should be https://id.awsetc/stage/
    // apigateway is listening for stage/{subscription_id}
    let callback_url = format!("{}{}", base_invoke_url, &subscription.id.to_string());

    let lease = (60 * 60 * 18).to_string();

    let params = {
        let mut p = HashMap::new();
        p.insert("hub.mode", "subscribe");
        p.insert("hub.topic", &subscription.topic_url);
        p.insert("hub.callback", &callback_url);
        p.insert("hub.verify", "sync");
        p.insert("hub.verify_token", verify_token);
        p.insert("hub.lease_seconds", &lease);
        p
    };

    let req = client.post(subscription.hub_url).form(&params);
    req.send().await?.error_for_status()?;

    Ok(())
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/lambda-runtime/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler(event: LambdaEvent<Event>) -> Result<(), Error> {
    // Extract some useful information from the request

    let client = reqwest::ClientBuilder::new().use_rustls_tls().build()?;
    let ssm_params = get_parameters(vec!["INVOKE_URL_SSM_PARAM", "VERIFY_TOKEN_PARAM"]).await?;

    let base_invoke_url = ssm_params[0].clone();
    let verify_token = ssm_params[1].clone();

    for record in event.payload.records {
        let item: Subscription = from_item(record.change.new_image)?;
        handle(item, &base_invoke_url, &verify_token, client.clone()).await?;
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
