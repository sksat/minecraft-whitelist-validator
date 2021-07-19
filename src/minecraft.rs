use serde::{Deserialize, Serialize};

pub type UserList = Vec<User>;

// this is NOT mojang::User !!!
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    name: String,
    uuid: String,
}
