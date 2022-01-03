use crate::mojang;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type UserList = Vec<User>;

// this is NOT mojang::User !!!
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub name: String,
    pub uuid: Uuid, // eab5e33d-8305-4e3d-aa03-8cf527faac7b
}

impl From<mojang::User> for User {
    fn from(u: mojang::User) -> User {
        let name = u.name;
        let uuid = &u.id;
        let uuid = Uuid::parse_str(uuid).unwrap_or_else(|_| panic!("parse UUID failed!: {}", uuid));
        User { name, uuid }
    }
}

impl User {
    pub async fn exist(&self) -> Result<bool, mojang::Error> {
        let uuid = mojang::name2uuid(&self.name).await?;
        if uuid.is_none() {
            return Ok(false);
        }
        Ok(self.uuid == uuid.unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use uuid::Uuid;

    #[test]
    fn user() {
        let sksat = minecraft::User {
            name: "sksat".to_string(),
            uuid: Uuid::parse_str("eab5e33d83054e3daa038cf527faac7b").unwrap(),
        };
        let json = serde_json::to_string(&sksat).unwrap();
        assert_eq!(
            json,
            r#"{"name":"sksat","uuid":"eab5e33d-8305-4e3d-aa03-8cf527faac7b"}"#
        )
    }

    #[tokio::test]
    async fn user_exist() {
        let sksat = minecraft::User {
            name: "sksat".to_string(),
            uuid: Uuid::parse_str("eab5e33d83054e3daa038cf527faac7b").unwrap(),
        };

        assert!(sksat.exist().await.unwrap());
    }
}
