#[allow(unused)]
use crate::routes::database_source::tasks::{self, Entity as Tasks};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
};
use sea_orm::Set;
#[allow(unused)]
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryParam {
    pub soft: bool,
}

pub async fn delete_tasks(
    Path(task_id): Path<i32>,
    State(database): State<DatabaseConnection>,
    Query(query_params): Query<QueryParam>,
) -> Result<(), StatusCode> {
    // ----------------------------------------------------------------
    // let squeeze_model_from_data = if let Some(task_from_database) = Tasks::find_by_id(task_id)
    //     .one(&database)
    //     .await
    //     .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    // // it means that the task NOT FOUND in database
    // {
    //     task_from_database.into_active_model()
    // } else {
    //     return Err(StatusCode::INTERNAL_SERVER_ERROR);
    // };

    // Tasks::delete(squeeze_model_from_data)
    //     .exec(&database)
    //     .await
    //     .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    // ---------------------------------------------------------------- is one way to delete a task

    // ----------------------------------------------------------------
    // Tasks::delete_by_id(task_id)
    //     .exec(&database)
    //     .await
    //     .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    // ---------------------------------------------------------------- is the other way(seems simpler than first one),if not found it's still response 200OK

    // ----------------------------------------------------------------
    // Tasks::delete_many()
    //     .filter(tasks::Column::Id.eq(task_id))
    //     .exec(&database)
    //     .await
    //     .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    // ---------------------------------------------------------------- when I want to delete multiple tasks, if not found it's still response 200OK
    // //! most important!!!!!!! MAKE SURE to set up the FILTER!!!!

    // ---------------------------------------------------------------- and this time we need to impl the soft delete function
    // if want to impl soft delete, I've to impl the "update" the "delete_at" to mark the task
    if query_params.soft {
        let mut task = if let Some(task_model) = Tasks::find_by_id(task_id)
            .one(&database)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        {
            task_model.into_active_model() // squeeze out the Model from the database, and next transfer the model into ActiveModel
        } else {
            return Err(StatusCode::NOT_FOUND);
        };

        let new = chrono::Utc::now();
        task.deleted_at = Set(Some(new.into())); // //! todo!!!!

        Tasks::update(task)
            .exec(&database)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    } else {
        Tasks::delete_by_id(task_id)
            .exec(&database)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    Ok(())
}
