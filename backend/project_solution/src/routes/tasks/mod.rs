use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension,
};
use sea_orm::{
    prelude::DateTimeWithTimeZone, ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter,
};
use serde::{Deserialize, Serialize};

use crate::{
    database::tasks::{self, ActiveModel, Entity as Task},
    database::users::Model as UserModel,
    utilities::app_error::AppError,
};

pub mod create_task;
pub mod create_task_extractor;
pub mod delete_task;
pub mod get_all_tasks;
pub mod get_one_task;
pub mod update_tasks;

#[derive(Serialize, Deserialize)]
pub struct RequestTaskContainer {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub priority: Option<Option<String>>,
    pub title: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub description: Option<Option<String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub completed_at: Option<Option<DateTimeWithTimeZone>>,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseTaskData {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<String>,
    pub completed_at: Option<DateTimeWithTimeZone>,
    // pub deleted_at: Option<DateTimeWithTimeZone>,
    // pub is_default: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseTaskContainer {
    pub data: ResponseTaskData,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseTasksContainer {
    pub data: Vec<ResponseTaskData>,
}

async fn task_with_user_checker(
    Path(task_id): Path<i32>,
    Extension(user): Extension<UserModel>,
    State(db): State<DatabaseConnection>,
) -> Result<ActiveModel, AppError> {
    let task = if let Some(task) = Task::find_by_id(task_id)
        .filter(tasks::Column::UserId.eq(user.id))
        .one(&db)
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

pub async fn save_task(
    task: ActiveModel,
    State(db): State<DatabaseConnection>,
) -> Result<(), AppError> {
    task.save(&db).await.map_err(|error| {
        eprintln!("database save error field{:?}", error);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "database save something went wrong",
        )
    })?;
    Ok(())
}
