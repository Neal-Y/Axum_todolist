use super::app_error::AppError;
use axum::http::StatusCode;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: usize,
    username: String,
}

// 因為secret是一個跟資料庫連接一次拿到資料就可以存起來的所以不用『每次』都尋找環境變量
pub fn create_jwt_token(secret: &str, username: String) -> Result<String, AppError> {
    let exp = (Utc::now() + Duration::seconds(20)).timestamp() as usize;

    let header = Header::new(Algorithm::HS256);
    let claims = Claims { exp, username };
    let key = EncodingKey::from_secret(secret.as_bytes());

    encode(&header, &claims, &key).map_err(|error| {
        eprintln!("failed to create jwt token {}", error); // 這是給伺服器看的也就是我們自己
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "plz try again") // 這是給客戶端看的
    })
}

pub fn verify_jwt_token(secret: &str, token: &str) -> Result<bool, AppError> {
    let key = DecodingKey::from_secret(secret.as_bytes());
    let validation = Validation::new(Algorithm::HS256);
    decode::<Claims>(token, &key, &validation)
        .map_err(|error| match error.kind() {
            jsonwebtoken::errors::ErrorKind::InvalidToken
            | jsonwebtoken::errors::ErrorKind::InvalidSignature
            | jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                AppError::new(StatusCode::UNAUTHORIZED, "not authenticated!")
            }
            _ => {
                eprintln!("{:?}", error);
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "something went wrong")
            }
        })
        .map(|_| true)
}
