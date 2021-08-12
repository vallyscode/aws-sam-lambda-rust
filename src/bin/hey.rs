use greetings::make_hey;
use lambda_http::{
    handler,
    lambda_runtime::{self, Context, Error},
    IntoResponse, Request, RequestExt, Response,
};

use serde::Serialize;

#[derive(Serialize)]
struct Hey {
    message: String,
}

impl Hey {
    pub fn new(message: String) -> Self {
        Hey { message }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(handler(func)).await?;
    Ok(())
}

async fn func(event: Request, _: Context) -> Result<impl IntoResponse, Error> {
    let params = event.query_string_parameters();
    let result = match params.get("name") {
        Some(name) => serde_json::to_string(&Hey::new(make_hey(name)))?.into_response(),
        _ => Response::builder()
            .status(400)
            .body("name is required".into())?,
    };
    Ok(result)
}
