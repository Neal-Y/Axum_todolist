use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub async fn custom_response() -> Response {
    (StatusCode::CREATED, "hello, this response".to_string()).into_response()
}
