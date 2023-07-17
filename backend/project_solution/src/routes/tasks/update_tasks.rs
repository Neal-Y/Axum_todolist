use crate::{
    database::{
        tasks::{self, Entity as Task},
        users::Model as UserModel,
    },
    utilities::{app_error::AppError, app_state::AppState},
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension,
};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, Set};

async fn mark_completion(
    completed: bool,
    Path(task_id): Path<i32>,
    Extension(user): Extension<UserModel>,
    State(app_state): State<AppState>,
) -> Result<(), AppError> {
    let mut task = if let Some(task) = Task::find_by_id(task_id)
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

    task.completed_at = if completed {
        Set(Some(Utc::now().into()))
    } else {
        Set(None)
    };

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
    path: Path<i32>,
    user: Extension<UserModel>,
    app_state: State<AppState>,
) -> Result<(), AppError> {
    mark_completion(true, path, user, app_state).await
}

pub async fn mark_uncompleted(
    path: Path<i32>,
    user: Extension<UserModel>,
    app_state: State<AppState>,
) -> Result<(), AppError> {
    mark_completion(false, path, user, app_state).await
}

// todo:refactor completed and uncompleted
// todo:implement update all task
