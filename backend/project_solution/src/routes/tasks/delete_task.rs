use axum::{
    extract::{Path, State},
    Extension,
};
use chrono::Utc;
use sea_orm::Set;

use crate::{
    database::users::Model as UserModel,
    utilities::{app_error::AppError, app_state::AppState},
};

use super::{save_task, task_with_user_checker};

pub async fn delete_task(
    Path(task_id): Path<i32>,
    Extension(user): Extension<UserModel>,
    State(app_state): State<AppState>,
) -> Result<(), AppError> {
    let mut task =
        task_with_user_checker(Path(task_id), Extension(user), State(&app_state.database)).await?;
    task.deleted_at = Set(Some(Utc::now().fixed_offset()));
    save_task(task, State(&app_state.database)).await?;
    Ok(())
}
