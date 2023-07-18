use axum::{
    async_trait,
    body::HttpBody,
    extract::FromRequest,
    http::{Request, StatusCode},
    BoxError, Json, RequestExt,
};

use serde::Deserialize;
use validator::Validate;

use crate::utilities::app_error::AppError;

#[derive(Debug, Validate, Deserialize)]
pub struct ValidateCreateTask {
    #[validate(length(min = 1, max = 1))]
    pub priority: Option<String>,
    #[validate(required(message = "missing task title"))]
    pub title: Option<String>,
    pub description: Option<String>,
    // pub deleted_at: Option<DateTimeWithTimeZone>,
    pub is_default: Option<bool>,
}

#[async_trait]
impl<S, B> FromRequest<S, B> for ValidateCreateTask
where
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(
        req: Request<B>,
        _state: &S,
    ) -> Result<ValidateCreateTask, Self::Rejection> {
        //? 這是是在將我的req中使用extract用turbofish也就是::<>的方式取出Json<ValidateCreateTask>，丟到Json包裹的task變數中，並且map_err一下

        let Json(task) = req
            .extract::<Json<ValidateCreateTask>, _>()
            .await
            .map_err(|error| {
                eprintln!("Error extracting new task: {:?}", error);
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Something went wrong, please try again",
                )
            })?;

        //? 接著才是重頭戲，我們使用.validate()函數『驗證』剛剛在struct中下的限制指令

        if let Err(errors) = task.validate() {
            let field_errors = errors.field_errors();
            if let Some(error_message) = field_errors
                .values() // 抓出field_errors中鍵值對的『值』並且返回一個迭代器 ps:取『鍵』.key()
                .flat_map(|errors| errors.iter()) // 扁平化，
                .find_map(|error| error.message.clone().map(|cow| cow.to_string()))
            {
                return Err(AppError::new(StatusCode::BAD_REQUEST, error_message));
            }
        }
        Ok(task)
    }
}
