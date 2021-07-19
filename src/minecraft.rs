use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type UserList = Vec<User>;

// this is NOT mojang::User !!!
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub name: String,
    pub uuid: Uuid, // eab5e33d-8305-4e3d-aa03-8cf527faac7b
}

impl From<crate::mojang::User> for User {
    fn from(u: crate::mojang::User) -> User {
        User {
            name: u.name,
            uuid: Uuid::parse_str(&u.id).unwrap(),
        }
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
}
