use std::collections::HashMap;

use reqwest::{self, StatusCode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug)]
pub enum Error {
    Server,
    Reqwest(reqwest::Error),
    Unknown,
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Error {
        Error::Reqwest(e)
    }
}

// https://wiki.vg/Mojang_API

type ApiStatus = HashMap<String, ApiStatusType>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ApiStatusType {
    Red,
    Green,
}

pub type UserList = Vec<User>;

// this is NOT minecraft::User !!!
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub name: String,
    pub id: String, // eab5e33d83054e3daa038cf527faac7b
}

pub async fn api_status() -> Result<Vec<ApiStatus>, Error> {
    let res = reqwest::get("https://status.mojang.com/check")
        .await?
        .text()
        .await?;

    let st = serde_json::from_str(&res).unwrap();

    Ok(st)
}

pub async fn name2user(uname: &str) -> Result<Option<User>, Error> {
    if uname.len() == 0 {
        return Ok(None);
    }

    let res = reqwest::get(format!(
        "https://api.mojang.com/users/profiles/minecraft/{}",
        uname
    ))
    .await?;

    println!("res: {:?}", res);

    match res.status() {
        StatusCode::OK => {}
        StatusCode::NO_CONTENT => return Ok(None),
        _ => return Err(Error::Server),
    }

    let res = res.text().await?;

    let user: User = serde_json::from_str(&res).unwrap();
    Ok(Some(user))
}

pub async fn name2uuid(uname: &str) -> Result<Option<Uuid>, Error> {
    let user = name2user(uname).await?;
    if let Some(u) = user {
        let u: crate::minecraft::User = u.into();
        Ok(Some(u.uuid))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use uuid::Uuid;

    #[tokio::test]
    async fn status() {
        let st = mojang::api_status().await.unwrap();

        println!("{:?}", st);
    }

    #[tokio::test]
    async fn name2uuid() {
        let empty = mojang::name2uuid("").await.unwrap();
        assert!(empty.is_none());

        let sksat_uuid = Uuid::parse_str("eab5e33d-8305-4e3d-aa03-8cf527faac7b").unwrap();

        let sksat = mojang::name2uuid("sksat").await.unwrap();
        assert_eq!(sksat, Some(sksat_uuid));
    }
}
