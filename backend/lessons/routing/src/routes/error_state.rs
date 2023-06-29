use axum::http::StatusCode;

pub async fn i_am_teapot() -> Result<(), StatusCode> {
    Err(StatusCode::IM_A_TEAPOT)
}
