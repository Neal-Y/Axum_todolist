use axum::http::StatusCode;
use chrono::{Duration, Utc};
use dotenvy_macro::dotenv;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use super::app_error::AppError;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    iat: usize, //? issued at : jwt 發放時間(戳記)
    exp: usize, //? expiration time : 超過時間就無效
}

pub fn create_jwt() -> Result<String, StatusCode> {
    let now = Utc::now();
    let expiration_time = now + Duration::seconds(20);

    let claims = Claims {
        iat: now.timestamp() as usize,
        exp: expiration_time.timestamp() as usize,
    };
    let secret = dotenv!("JWT_SECRET");
    let key = EncodingKey::from_secret(secret.as_bytes());

    encode(&Header::new(Algorithm::HS512), &claims, &key)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn verify_jwt(token: &str) -> Result<(), AppError> {
    let secret = dotenv!("JWT_SECRET");
    let key = DecodingKey::from_secret(secret.as_bytes());
    decode::<Claims>(token, &key, &Validation::new(Algorithm::HS512)).map_err(
        |error| match error.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                AppError::new(StatusCode::UNAUTHORIZED, "expired! plz login again")
            }
            _ => AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "server's error"),
        },
    )?;
    Ok(())
}
