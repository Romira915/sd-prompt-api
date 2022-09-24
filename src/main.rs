use async_once_cell::{Lazy as AsyncLazy, OnceCell};
use aws_sdk_dynamodb::{model::AttributeValue, Client};
use fancy_regex::Regex;
use lambda_http::{
    http::{request, Method},
    request::RequestContext,
    run, service_fn, Body, Error, Request, RequestExt, Response,
};
use log::LevelFilter;
use once_cell::sync::Lazy;
use sd_prompt_api::CLIENT;
use serde::{Deserialize, Serialize};
use serde_json::json;
use simplelog::{CombinedLogger, ConfigBuilder, TermLogger};

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let resource_path = if let RequestContext::ApiGatewayV2(http_context) = event.request_context()
    {
        http_context.http.path.unwrap()
    } else {
        unreachable!()
    };

    static RATE: Lazy<Regex> = Lazy::new(|| Regex::new("^/\\w+(?=/)").unwrap());
    // stage名を削除
    let resource_path = RATE.replace(&resource_path, "");

    let resp = match (event.method(), resource_path.as_ref()) {
        (&Method::GET, "/prompt") => {
            log::debug!("GET /prompt");
            todo!()
        }
        (&Method::POST, "/prompt") => {
            log::debug!("POST /prompt");
            todo!()
        }
        _ => Response::builder()
            .status(200)
            .header("content-type", "application/json")
            .body(
                json!({
                    "message": "This method or path is not support."
                })
                .to_string()
                .into(),
            )
            .map_err(Box::new)?,
    };

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    CombinedLogger::init(vec![TermLogger::new(
        if cfg!(debug_assertions) {
            LevelFilter::Debug
        } else {
            LevelFilter::Info
        },
        ConfigBuilder::default().build(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Always,
    )])
    .unwrap();

    if cfg!(debug_assertions) {
        log::info!("Debug mode");
    } else {
        log::info!("Release mode");
    }

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    CLIENT
        .get_or_init(async {
            let shared_config = aws_config::load_from_env().await;
            Client::new(&shared_config)
        })
        .await;

    run(service_fn(function_handler)).await
}
