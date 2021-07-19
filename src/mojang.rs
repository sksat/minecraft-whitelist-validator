use std::collections::HashMap;

use reqwest;
use serde::{Deserialize, Serialize};

// https://wiki.vg/Mojang_API

type ApiStatus = HashMap<String, ApiStatusType>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ApiStatusType {
    Red,
    Green,
}

pub async fn api_status() -> Result<Vec<ApiStatus>, reqwest::Error> {
    let res = reqwest::get("https://status.mojang.com/check")
        .await?
        .text()
        .await?;

    let st = serde_json::from_str(&res).unwrap();

    Ok(st)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[tokio::test]
    async fn status() {
        let st = mojang::api_status().await.unwrap();

        println!("{:?}", st);
    }
}
