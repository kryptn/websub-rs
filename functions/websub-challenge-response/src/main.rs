use core::time;
use std::{
    env,
    time::{Duration, SystemTime},
};

use lambda_http::{run, service_fn, Error, IntoResponse, Request, RequestExt, Response};
use rusoto_core::Region;
use rusoto_ssm::{GetParametersRequest, Ssm, SsmClient};
use tracing_subscriber::EnvFilter;
use uuid::Uuid;
use websub::{SubscriptionLease, WebsubClient};

async fn get_parameters(param_envs: Vec<&str>) -> Result<Vec<String>, Error> {
    let ssm_client = SsmClient::new(Region::default());
    let names: Vec<String> = param_envs
        .iter()
        .map(|v| env::var(v).expect("required env"))
        .collect();

    let req = GetParametersRequest {
        names,
        with_decryption: None,
    };
    let resp = ssm_client.get_parameters(req).await?;

    let mut out = Vec::new();

    if let Some(parameters) = resp.parameters {
        let p = parameters.iter().map(|p| p.clone().value.unwrap());
        out.extend(p);
    }

    Ok(out)
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/lambda-http/examples
async fn function_handler(event: Request) -> Result<impl IntoResponse, Error> {
    let parameters = get_parameters(vec!["VERIFY_TOKEN_PARAM"]).await?;
    let trusted_verify_token = parameters.first().unwrap();

    let query = event.query_string_parameters();

    let _mode = query
        .first("hub.mode")
        .expect("challenge must include mode");
    let _topic = query
        .first("hub.topic")
        .expect("challenge must include topic");
    let challenge = query
        .first("hub.challenge")
        .expect("challenge must include challenge")
        .to_string();
    let verify_token = query
        .first("hub.verify_token")
        .expect("challenge must include verify_token")
        .to_string();

    if &verify_token != trusted_verify_token {
        panic!("verify_token did not match expected value")
    }

    let lease_seconds = query
        .first("hub.lease_seconds")
        .expect("we are providing this value")
        .parse::<usize>()?;

    let path_params = event.path_parameters();

    let subscription_id = path_params
        .first("subscription_id")
        .expect("we are providing this value");
    let subscription_id = Uuid::parse_str(subscription_id)?;

    let client = WebsubClient::default().await;

    let lease_seconds = lease_seconds - (lease_seconds / 20);
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let expiry = now + Duration::from_secs(lease_seconds as u64);
    let expiry = expiry.as_secs() as usize;

    let lease = SubscriptionLease::new(subscription_id, expiry);

    client.create_lease(&lease).await?;

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(challenge)
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
