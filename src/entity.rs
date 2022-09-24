use anyhow::{bail, Context};
use std::{
    collections::HashMap,
    convert::TryFrom,
    ops::{Index, IndexMut},
};

use aws_sdk_dynamodb::model::AttributeValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub(crate) struct Prompt {
    pub(crate) prompt_id: Option<u32>,
    pub(crate) prompt: String,
}

impl TryFrom<&HashMap<String, AttributeValue>> for Prompt {
    type Error = anyhow::Error;
    fn try_from(value: &HashMap<String, AttributeValue>) -> Result<Self, Self::Error> {
        let converted = match (value.get("prompt_id"), value.get("prompt")) {
            (Some(prompt_id), Some(prompt)) => {
                let prompt_id = if let AttributeValue::N(id) = prompt_id {
                    Some(id.parse().context("Failed to parse prompt_id")?)
                } else {
                    bail!("Not match user_id AttributeValue.");
                };
                let prompt = if let AttributeValue::S(p) = prompt {
                    p.to_string()
                } else {
                    bail!("Not match name AttributeValue.");
                };

                Ok(Self { prompt_id, prompt })
            }
            (None, Some(prompt)) => {
                let prompt = if let AttributeValue::S(p) = prompt {
                    p.to_string()
                } else {
                    bail!("Not match name AttributeValue.");
                };
                Ok(Self {
                    prompt_id: None,
                    prompt,
                })
            }
            _ => bail!("Not Found key item"),
        };

        converted
    }
}
