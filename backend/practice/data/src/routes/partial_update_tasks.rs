use crate::routes::database_source::tasks::{self, Entity as Tasks};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{
    prelude::DateTimeWithTimeZone, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel,
    QueryFilter, Set,
};
use serde::Deserialize;

#[derive(Deserialize)] // to set up the struct can be Deserialized, because when I can some data from the request body, I need to Deserialize into my struct.
pub struct RequestTask {
    pub id: Option<i32>,

    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    pub priority: Option<Option<String>>,

    pub title: Option<String>,

    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    pub completed_at: Option<Option<DateTimeWithTimeZone>>,

    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    pub description: Option<Option<String>>,

    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    pub deleted_at: Option<Option<DateTimeWithTimeZone>>,
}

pub async fn partial_update(
    Path(task_id): Path<i32>,
    State(database): State<DatabaseConnection>, // is middleware to get the info about the database
    Json(update_msg): Json<RequestTask>, // is request body's data which need to be written in database
) -> Result<(), StatusCode> {
    let mut db_partial_tasks = if let Some(task) = Tasks::find_by_id(task_id)
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        task.into_active_model()
    } else {
        return Err(StatusCode::NOT_FOUND);
    };

    if let Some(priority) = update_msg.priority {
        db_partial_tasks.priority = Set(priority);
    };

    if let Some(title) = update_msg.title {
        db_partial_tasks.title = Set(title);
    };

    if let Some(completed_at) = update_msg.completed_at {
        db_partial_tasks.completed_at = Set(completed_at);
    };

    if let Some(description) = update_msg.description {
        db_partial_tasks.description = Set(description);
    };

    if let Some(deleted_at) = update_msg.deleted_at {
        db_partial_tasks.deleted_at = Set(deleted_at);
    };

    Tasks::update(db_partial_tasks)
        .filter(tasks::Column::Id.eq(task_id))
        .exec(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
