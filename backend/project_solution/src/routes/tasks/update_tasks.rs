use crate::{
    database::{
        tasks::{self, ActiveModel, Entity as Task},
        users::Model as UserModel,
    },
    utilities::{app_error::AppError, app_state::AppState},
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, Set};

use super::RequestTaskContainer;

async fn mark_completion(
    Path(task_id): Path<i32>,
    Extension(user): Extension<UserModel>,
    State(app_state): State<&AppState>,
) -> Result<ActiveModel, AppError> {
    let task = if let Some(task) = Task::find_by_id(task_id)
        .filter(tasks::Column::UserId.eq(user.id))
        .one(&app_state.database)
        .await
        .map_err(|error| {
            eprintln!("update error field{:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "something went wrong")
        })? {
        task.into_active_model()
    } else {
        return Err(AppError::new(StatusCode::NOT_FOUND, "not found"));
    };

    Ok(task)
}

pub async fn save_task(task: ActiveModel, app_state: State<&AppState>) -> Result<(), AppError> {
    task.save(&app_state.database).await.map_err(|error| {
        eprintln!("database save error field{:?}", error);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "database save something went wrong",
        )
    })?;
    Ok(())
}

pub async fn mark_completed(
    Path(path): Path<i32>,
    Extension(user): Extension<UserModel>,
    State(app_state): State<AppState>,
) -> Result<(), AppError> {
    let mut task = mark_completion(Path(path), Extension(user), State(&app_state)).await?;
    task.completed_at = Set(Some(Utc::now().into()));
    save_task(task, State(&app_state)).await?;
    Ok(())
}

pub async fn mark_uncompleted(
    Path(path): Path<i32>,
    Extension(user): Extension<UserModel>,
    State(app_state): State<AppState>,
) -> Result<(), AppError> {
    let mut task = mark_completion(Path(path), Extension(user), State(&app_state)).await?;
    task.completed_at = Set(None);
    save_task(task, State(&app_state)).await?;
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
    let mut task = mark_completion(Path(task_id), Extension(user), State(&app_state)).await?;
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
    save_task(task, State(&app_state)).await?;
    Ok(())
}
