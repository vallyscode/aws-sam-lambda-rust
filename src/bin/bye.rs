use greetings::make_bye;
use lambda_http::{
    handler,
    lambda_runtime::{self, Context, Error},
    IntoResponse, Request, RequestExt, Response,
};

use serde::Serialize;

#[derive(Serialize)]
struct Bye {
    message: String,
}

impl Bye {
    pub fn new(message: String) -> Self {
        Bye { message }
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
        Some(name) => serde_json::to_string(&Bye::new(make_bye(name)))?.into_response(),
        _ => Response::builder()
            .status(400)
            .body("name is required".into())?,
    };
    Ok(result)
}
