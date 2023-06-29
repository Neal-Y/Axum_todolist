use axum::{extract::State, http::StatusCode, Extension};
use sea_orm::{ActiveModelTrait, IntoActiveModel, Set};

use crate::{
    database::users,
    utilities::{app_error::AppError, app_state::AppState},
};

pub async fn user_logout(
    State(app_state): State<AppState>,
    Extension(user): Extension<users::Model>,
) -> Result<StatusCode, AppError> {
    let mut user = user.into_active_model();

    user.token = Set(None);

    user.save(&app_state.database).await.map_err(|error| {
        eprintln!("Error saving {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "try again")
    })?;

    Ok(StatusCode::OK)
}

// //! 搞清楚是因為中間件的關係所以我是用Extension去取得user?
