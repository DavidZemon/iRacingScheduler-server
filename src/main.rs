use lambda_runtime::{Error, LambdaEvent, run, service_fn};
use log::Level;
use serde_json::{json, Value};
use simple_logger;
use tokio::main;

#[main]
async fn main() -> Result<(), Error> {
    simple_logger::init_with_level(Level::Info)?;
    run(service_fn(handler)).await
}

async fn handler(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (event, _context) = event.into_parts();
    let first_name = event["firstName"].as_str().unwrap_or("world");

    Ok(json!({ "message": format!("Hello, {}!", first_name) }))
}
