use std::collections::HashMap;

use reqwest::{self, StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum Error {
    EmptyUser,
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
    name: String,
    id: String,
}

pub async fn api_status() -> Result<Vec<ApiStatus>, Error> {
    let res = reqwest::get("https://status.mojang.com/check")
        .await?
        .text()
        .await?;

    let st = serde_json::from_str(&res).unwrap();

    Ok(st)
}

pub async fn name2uuid(uname: &str) -> Result<Option<String>, Error> {
    if uname.len() == 0 {
        return Err(Error::EmptyUser);
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

    Ok(Some(user.id))
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[tokio::test]
    async fn status() {
        let st = mojang::api_status().await.unwrap();

        println!("{:?}", st);
    }

    #[tokio::test]
    async fn name2uuid() {
        let empty = mojang::name2uuid("").await;
        //assert_matches!(mojang::Error::EmptyUser, empty.err().unwrap());
        {
            let _e = empty.err().unwrap();
            assert!(matches!(mojang::Error::EmptyUser, _e));
        }

        let mut sksat_uuid = "eab5e33d-8305-4e3d-aa03-8cf527faac7b".to_string();
        sksat_uuid.retain(|c| c != '-');

        let sksat = mojang::name2uuid("sksat").await.unwrap();
        assert_eq!(sksat, Some(sksat_uuid));
    }
}
