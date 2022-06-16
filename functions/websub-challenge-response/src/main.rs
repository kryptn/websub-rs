use lambda_http::{run, service_fn, Error, IntoResponse, Request, RequestExt, Response};

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/lambda-http/examples
async fn function_handler(event: Request) -> Result<impl IntoResponse, Error> {
    // Extract some useful information from the request

    let query = event.query_string_parameters();

    let mode = query
        .first("hub.mode")
        .expect("challenge must include mode");
    let topic = query
        .first("hub.topic")
        .expect("challenge must include topic");
    let challenge = query
        .first("hub.challenge")
        .expect("challenge must include challenge")
        .to_string();
    let lease_seconds = query.first("hub.lease_seconds");

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
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
