use axum::extract::FromRef;
use sea_orm::DatabaseConnection;

use super::token_wrapper::TokenWrapper;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub database: DatabaseConnection,
    pub jwt_secret: TokenWrapper,
}
