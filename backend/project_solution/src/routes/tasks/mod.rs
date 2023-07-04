use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};

pub mod create_task;
pub mod create_task_extractor;

#[derive(Serialize, Deserialize)]
pub struct RequestTaskContainer {
    pub priority: Option<String>,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseTaskData {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<String>,
    pub completed_at: Option<DateTimeWithTimeZone>,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseTaskContainer {
    data: ResponseTaskData,
}
