use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::database::{
    tasks::{self, Entity as Task},
    users,
};
use crate::utilities::{app_error::AppError, app_state::AppState};

use super::{ResponseTaskContainer, ResponseTaskData};

pub async fn get_task(
    Path(task_id): Path<i32>,
    State(app_state): State<AppState>,
    Extension(user): Extension<users::Model>,
) -> Result<Json<ResponseTaskContainer>, AppError> {
    let task = Task::find_by_id(task_id)
        .filter(tasks::Column::UserId.eq(Some(user.id)))
        .one(&app_state.database)
        .await
        .map_err(|error| {
            eprintln!("Error getting {:?}", error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "we got an error when attempting to load your task",
            )
            // not found user
        })?;

    if let Some(task) = task {
        Ok(Json(ResponseTaskContainer {
            data: ResponseTaskData {
                id: task.id,
                title: task.title,
                description: task.description,
                priority: task.priority,
                completed_at: task.completed_at,
                // is_default: task.is_default,
            },
        }))
    } else {
        Err(AppError::new(StatusCode::NOT_FOUND, "not found"))
        // not found task
    }
}
