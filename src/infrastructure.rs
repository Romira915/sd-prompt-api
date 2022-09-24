use crate::{entity::Prompt, CLIENT, CONFIG};
use anyhow::{Context, Result};
use aws_sdk_dynamodb::model::AttributeValue;

pub(crate) async fn get_prompt(id: u32) -> Result<Prompt> {
    let item_output = CLIENT
        .get()
        .unwrap()
        .get_item()
        .table_name(&CONFIG.table_name)
        .key("prompt_id", AttributeValue::N(id.to_string()))
        .send()
        .await
        .context("Failed to get_prompt send()")?;
    let item_hash_map = item_output.item().context("Failed to get_prompt item()")?;

    let prompt = Prompt::try_from(item_hash_map)?;

    Ok(prompt)
}

pub(crate) async fn update_prompt(prompt: &Prompt) -> Result<()> {
    let update_item = CLIENT
        .get()
        .unwrap()
        .update_item()
        .table_name(&CONFIG.table_name)
        .key(
            "prompt_id",
            AttributeValue::N(prompt.prompt_id.unwrap_or_default().to_string()),
        )
        .update_expression("SET prompt=:prompt")
        .expression_attribute_values(":prompt", AttributeValue::S(prompt.prompt.to_string()));

    update_item
        .send()
        .await
        .context("Failed to update_prompt send()")?;

    Ok(())
}
