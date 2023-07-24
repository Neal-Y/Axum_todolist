use crate::routes::database_source::tasks::{self, Entity as Tasks};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use sea_orm::{
    prelude::DateTimeWithTimeZone, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    QueryFilter,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct RequestTask {
    id: i32,
    priority: Option<String>,
    title: String,
    description: Option<String>,
    deleted_at: Option<DateTimeWithTimeZone>,
    user_id: Option<i32>,
}

pub async fn get_task(
    Path(task_id): Path<i32>,
    State(database): State<DatabaseConnection>,
) -> Result<Json<RequestTask>, StatusCode> {
    let task = Tasks::find_by_id(task_id)
        .filter(tasks::Column::DeletedAt.is_null())
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(task) = task {
        Ok(Json(RequestTask {
            id: task.id,
            priority: task.priority,
            title: task.title,
            description: task.description,
            deleted_at: task.deleted_at,
            user_id: task.user_id,
        }))
    } else {
        Err(StatusCode::IM_A_TEAPOT)
    }
}

//? 存Query 參數
#[derive(Deserialize)]
pub struct GetTaskQueryParams {
    priority: Option<String>,
}

pub async fn get_tasks(
    State(database): State<DatabaseConnection>,
    Query(filter_params): Query<GetTaskQueryParams>, //? this is to list a specific filter of tasks
) -> Result<Json<Vec<RequestTask>>, StatusCode> {
    // let tasks_table = Tasks::find()
    //     .all(&database)
    //     .await
    //     .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);

    // if let Ok(tasks_table) = tasks_table {
    //     let request_tasks: Vec<RequestTask> = tasks_table
    //         .into_iter()
    //         .map(|task| RequestTask {
    //             id: task.id,
    //             priority: task.priority,
    //             title: task.title,
    //             description: task.description,
    //         })
    //         .collect();
    //     return Ok(Json(request_tasks));
    // } else {
    //     return Err(StatusCode::NOT_FOUND);
    // }

    //? 另一種解法
    // let table_task = Tasks::find()
    //     .all(&database)
    //     .await
    //     .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    //     .into_iter()
    //     .map(|db_tasks| RequestTask {
    //         id: db_tasks.id,
    //         priority: db_tasks.priority,
    //         title: db_tasks.title,
    //         description: db_tasks.description,
    //     })
    //     .collect();
    // Ok(Json(table_task))

    //----------------------------------------------------------------

    // priority_filter is a kind of functionalization the condition into the filter parameters
    let mut task_filter = Condition::all().add(tasks::Column::DeletedAt.is_null());
    //? to plus a condition into the filter

    if let Some(priority) = filter_params.priority {
        task_filter = if priority.is_empty() {
            task_filter.add(tasks::Column::Priority.is_null())
        } else {
            task_filter.add(tasks::Column::Priority.eq(priority))
        }
    }

    //? I want to filter that when the task's delete_at is null (delete_tasks)

    let table_task = Tasks::find()
        .filter(task_filter)
        .all(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|db_tasks| RequestTask {
            id: db_tasks.id,
            priority: db_tasks.priority,
            title: db_tasks.title,
            description: db_tasks.description,
            deleted_at: db_tasks.deleted_at,
            user_id: db_tasks.user_id,
        })
        .collect();
    Ok(Json(table_task))
}
