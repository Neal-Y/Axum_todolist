use axum::{extract::State, http::StatusCode, Extension, Json};
use sea_orm::{ActiveModelTrait, Set, TryIntoModel};

use crate::{
    database::{tasks, users::Model as UserModel},
    utilities::{app_error::AppError, app_state::AppState},
};

use super::{create_task_extractor::ValidateCreateTask, ResponseTaskContainer, ResponseTaskData};

pub async fn new_task(
    Extension(user): Extension<UserModel>,
    State(app_state): State<AppState>,
    task_info: ValidateCreateTask,
) -> Result<(StatusCode, Json<ResponseTaskContainer>), AppError> {
    let new_task = tasks::ActiveModel {
        priority: Set(task_info.priority),
        title: Set(task_info.title.unwrap()),
        description: Set(task_info.description),
        user_id: Set(Some(user.id)),
        // deleted_at: Set(task_info.deleted_at),
        // is_default: Set(task_info.is_default),
        ..Default::default()
    };

    let task = new_task
        .save(&app_state.database)
        .await
        .map_err(|error| {
            eprintln!("Error saving {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "error saving")
        })?
        .try_into_model()
        .map_err(|error| {
            eprintln!("Error into model {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "error into model")
        })?;

    let response = ResponseTaskData {
        id: task.id,
        title: task.title,
        description: task.description,
        priority: task.priority,
        completed_at: task.completed_at.map(|time| time.to_string()),
        // deleted_at: task.deleted_at,
        // is_default: task.is_default,
    };
    Ok((
        StatusCode::CREATED,
        Json(ResponseTaskContainer { data: response }),
    ))
}
