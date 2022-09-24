use anyhow::{Context, Result};
use lambda_http::{Body, Request, Response};
use serde_json::json;

use crate::{
    entity::Prompt,
    infrastructure::{get_prompt, update_prompt},
};

pub async fn get_prompt_handler(event: &Request) -> Result<Response<Body>> {
    let prompt = get_prompt(0).await?;

    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(
            serde_json::to_string(&prompt)
                .context("Failed to get_prompt_handler serd_json::to_string")?
                .into(),
        )
        .context("fialed to get_prompt_handler Response body.")?;

    Ok(resp)
}

pub async fn post_prompt_handler(event: &Request) -> Result<Response<Body>> {
    let prompt = String::from_utf8(event.body().to_vec()).context("Failed to vec to String")?;
    let prompt: Prompt = serde_json::from_str(&prompt).context("Failed to str to json")?;

    update_prompt(&prompt).await?;

    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(json!({"message":"success"}).to_string().into())
        .context("Failed to Response body.")?;

    Ok(resp)
}
