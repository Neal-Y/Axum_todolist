use serde::{Deserialize, Serialize};

pub mod create_task;
pub mod create_task_extractor;
pub mod get_all_tasks;
pub mod get_one_task;
pub mod update_tasks;

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
    pub completed_at: Option<String>,
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
