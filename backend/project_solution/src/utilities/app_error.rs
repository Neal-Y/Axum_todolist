use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

//? 先定義容器以及錯誤處理的產生器
#[derive(Debug)]
pub struct AppError {
    code: StatusCode,
    message: String,
}

#[allow(dead_code)]
impl AppError {
    pub fn new(code: StatusCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

//? 把他變成正規可以回傳的
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            self.code,
            Json(ReturnMsgToClient {
                error: self.message.clone(),
            }),
        )
            .into_response()
    }
}

//? 丟回去客戶端的容器
#[derive(Serialize)]
struct ReturnMsgToClient {
    error: String,
}
