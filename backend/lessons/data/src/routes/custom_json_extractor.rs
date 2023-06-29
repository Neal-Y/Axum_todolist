//? 這是一個關於如何自定義extractor的檔案

use axum::{
    async_trait,
    body::HttpBody,
    extract::FromRequest,
    http::{Request, StatusCode},
    BoxError, Json, RequestExt,
};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct RequestUser {
    #[validate(email(message = "plz input email"))]
    pub username: String,
    #[validate(length(min = 8, message = "min length"))]
    pub password: String,
}

// 自定義你的Extractor，需要實現(impl)**FromRequest**這個trait

/*
   extract 方法是 FromRequest trait 的一部分，
   其作用是從請求的各個部分（如路徑、查詢參數、主體等）中提取出需要的資訊。
   在這裡，extract 方法用於從請求的主體中提取出一個 Json<RequestUser>。
*/

#[async_trait]
impl<S, B> FromRequest<S, B> for RequestUser
where
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String); //? 這裡就是為from_request實現關聯類型Rejection應用到下面的返回類型

    async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(user) = req
            .extract::<Json<RequestUser>, _>()
            .await
            .map_err(|error| (StatusCode::BAD_REQUEST, format!("{}", error)))?;

        if let Err(error) = user.validate() {
            return Err((StatusCode::BAD_REQUEST, format!("{}", error)));
        };

        Ok(user)
    }
}

pub async fn json_extractor(user: RequestUser) {
    dbg!(user);
}
