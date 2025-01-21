use lambda_runtime::{run, service_fn, Error, LambdaEvent};

use serde::{Deserialize, Serialize};

// Input struct
#[derive(Deserialize)]
struct Request {
    name: String,
}

// Output struct
#[derive(Serialize)]
struct Response {
    req_id: String,
    payload: String,
}

// All the work is done in this handler function
async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    // Extract useful information from the request
    let name = event.payload.name;
    let payload_body = if name == "Marco" {
        "Polo".to_string()
    } else {
        "Nobody".to_string()
    };
    // tracing to debug inside the lambda console
    tracing::info!("name {}", name);

    // Prepare the response. Automatically serialized
    let resp = Response {
        req_id: event.context.request_id,
        payload: payload_body,
    };

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}