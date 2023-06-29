pub mod create_user;
pub mod login;
pub mod logout;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ResponseDataMsg {
    data: ResponseUserId,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseUserId {
    id: i32,
    username: String,
    token: String,
}

#[derive(Serialize, Deserialize)]
pub struct RequestUser {
    username: String,
    password: String,
}
