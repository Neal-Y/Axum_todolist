use crate::database::users::{self, Entity as Users};
use crate::utilities::hash_table::verify_hash_password;
use crate::utilities::jwt::create_jwt_token;
use crate::utilities::{app_error::AppError, app_state::AppState};
use axum::http::StatusCode;
use axum::{extract::State, Json};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, Set, TryIntoModel,
};

use super::{RequestUser, ResponseDataMsg, ResponseUserId};

pub async fn user_login(
    State(app_state): State<AppState>,
    Json(login_info): Json<RequestUser>,
) -> Result<Json<ResponseDataMsg>, AppError> {
    let database_user = Users::find()
        .filter(users::Column::Username.eq(login_info.username))
        .one(&app_state.database)
        .await
        .map_err(|error| {
            eprintln!("{:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Invalid username")
        })?;

    // //! 資料庫沒找到所以database_user是none導致下面直接返回
    // //! 所以接下來請找到api.test.js是怎麼設定的

    if let Some(verified_user_data) = database_user {
        if !verify_hash_password(&login_info.password, &verified_user_data.password)? {
            return Err(AppError::new(StatusCode::UNAUTHORIZED, "Invalid password"));
        }

        let new_token =
            create_jwt_token(&app_state.jwt_secret.0, verified_user_data.username.clone())?;

        let mut update_user_data = verified_user_data.into_active_model();

        update_user_data.token = Set(Some(new_token));

        let save_verified_user_data =
            update_user_data
                .save(&app_state.database)
                .await
                .map_err(|e| {
                    eprintln!("{:?}", e);
                    AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error saving")
                })?;

        let finished_update = save_verified_user_data.try_into_model().map_err(|error| {
            eprintln!("{:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "plz try again")
        })?;

        let token = finished_update
            .token
            .ok_or_else(|| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "missing token"))?;

        let response = ResponseUserId {
            id: finished_update.id,
            username: finished_update.username,
            token,
        };

        Ok(Json(ResponseDataMsg { data: response }))
    } else {
        Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "incorrect username and/or password",
        ))
    }
}
