use super::app_error::AppError;
use axum::http::StatusCode;

const COST: u32 = 12;

pub fn hash_password(password: &str) -> Result<String, AppError> {
    bcrypt::hash(password, COST).map_err(|error| {
        eprintln!("hash_password went wrong {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Oops! Try again")
    })
}
pub fn verify_hash_password(password: &str, hash: &str) -> Result<bool, AppError> {
    bcrypt::verify(password, hash).map_err(|error| {
        eprintln!("verify_hash_password went wrong {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Oops! Try again")
    })
}
