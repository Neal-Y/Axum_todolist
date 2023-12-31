use crate::routes::database_source::tasks::{self, Entity as Tasks};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{
    prelude::DateTimeWithTimeZone, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestTask {
    pub id: Option<i32>,
    pub priority: Option<String>,
    pub title: String,
    pub completed_at: Option<DateTimeWithTimeZone>,
    pub description: Option<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
    pub user_id: Option<i32>,
    pub is_default: Option<bool>,
}

pub async fn atomic_update(
    Path(task_id): Path<i32>,
    State(database): State<DatabaseConnection>,
    Json(update_msg): Json<RequestTask>,
) -> Result<(), StatusCode> {
    let update_task = tasks::ActiveModel {
        id: Set(task_id),
        priority: Set(update_msg.priority),
        title: Set(update_msg.title),
        completed_at: Set(update_msg.completed_at),
        description: Set(update_msg.description),
        deleted_at: Set(update_msg.deleted_at),
        user_id: Set(update_msg.user_id),
        is_default: Set(update_msg.is_default),
    };

    Tasks::update(update_task)
        .filter(tasks::Column::Id.eq(task_id))
        .exec(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
