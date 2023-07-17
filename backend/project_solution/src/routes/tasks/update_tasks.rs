use crate::{
    database::users::Model as UserModel,
    utilities::{app_error::AppError, app_state::AppState},
};
use axum::{
    extract::{Path, State},
    Extension, Json,
};
use chrono::Utc;
use sea_orm::Set;

use super::{save_task, task_with_user_checker, RequestTaskContainer};

pub async fn mark_completed(
    Path(path): Path<i32>,
    Extension(user): Extension<UserModel>,
    State(app_state): State<AppState>,
) -> Result<(), AppError> {
    let mut task = task_with_user_checker(
        Path(path),
        Extension(user),
        State(app_state.database.clone()),
    )
    .await?;
    task.completed_at = Set(Some(Utc::now().into()));
    save_task(task, State(app_state.database)).await?;
    Ok(())
}

pub async fn mark_uncompleted(
    Path(path): Path<i32>,
    Extension(user): Extension<UserModel>,
    State(app_state): State<AppState>,
) -> Result<(), AppError> {
    let mut task = task_with_user_checker(
        Path(path),
        Extension(user),
        State(app_state.database.clone()),
    )
    .await?;
    task.completed_at = Set(None);
    save_task(task, State(app_state.database)).await?;
    Ok(())
}

// // todo:refactor completed and uncompleted
// // todo:implement update all task

pub async fn update_all_field_with_task(
    Path(task_id): Path<i32>,
    Extension(user): Extension<UserModel>,
    State(app_state): State<AppState>,
    Json(request_task_data): Json<RequestTaskContainer>,
) -> Result<(), AppError> {
    let mut task = task_with_user_checker(
        Path(task_id),
        Extension(user),
        State(app_state.database.clone()),
    )
    .await?;
    if let Some(title) = request_task_data.title {
        task.title = Set(title)
    };
    if let Some(priority) = request_task_data.priority {
        task.priority = Set(priority);
    }

    if let Some(completed_at) = request_task_data.completed_at {
        task.completed_at = Set(completed_at);
    }

    if let Some(description) = request_task_data.description {
        task.description = Set(description);
    }
    save_task(task, State(app_state.database)).await?;
    Ok(())
}
