use super::{RequestUser, ResponseDataMsg, ResponseUserId};
use crate::utilities::app_error::AppError;
use crate::utilities::app_state::AppState;
use crate::utilities::hash_table::hash_password;
use crate::{database::users, utilities::jwt::create_jwt_token};
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use sea_orm::{ActiveModelTrait, Set, TryIntoModel};

pub async fn create_user(
    State(app_state): State<AppState>,
    Json(request_data): Json<RequestUser>,
) -> Result<Json<ResponseDataMsg>, AppError> {
    let mut new_user = users::ActiveModel {
        ..Default::default()
    };
    new_user.username = Set(request_data.username.clone());
    new_user.password = Set(hash_password(&request_data.password)?);
    new_user.token = Set(Some(create_jwt_token(
        &app_state.jwt_secret.0,
        request_data.username,
    )?));

    let user = new_user
        .save(&app_state.database)
        .await
        .map_err(|error| {
            let error_msg = error.to_string();
            if error_msg
                .contains("duplicate key value violates unique constraint \"users_username_key\"")
            {
                eprintln!("{:?}", error);
                AppError::new(
                    StatusCode::BAD_REQUEST,
                    "Username already taken, try again with a different user name",
                )
            } else {
                eprintln!("{:?}", error);
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Something went wrong, plz try again",
                )
            }
        })?
        .try_into_model() // 因為需要取 user.id所以需要把ActiveModel 轉成Model
        .map_err(|error| {
            eprintln!("can't into model {}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error creating user")
        })?;

    Ok(Json(ResponseDataMsg {
        data: ResponseUserId {
            id: user.id,
            username: user.username,
            token: user.token.unwrap(),
        },
    }))
}
