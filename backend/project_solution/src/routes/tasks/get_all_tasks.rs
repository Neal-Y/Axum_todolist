use axum::{extract::State, http::StatusCode, Extension, Json};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{
    database::{
        tasks::{self, Entity as Tasks},
        users::Model as UserModel,
    },
    routes::tasks::ResponseTaskData,
    utilities::{app_error::AppError, app_state::AppState},
};

use super::ResponseTasksContainer;

pub async fn get_tasks(
    Extension(user): Extension<UserModel>,
    State(app_state): State<AppState>,
) -> Result<Json<ResponseTasksContainer>, AppError> {
    let filter_conditions = tasks::Column::UserId
        .eq(Some(user.id))
        .and(tasks::Column::DeletedAt.is_null());

    let table_task = Tasks::find()
        .filter(filter_conditions)
        .all(&app_state.database)
        .await
        .map_err(|error| {
            eprintln!("cant find user's task {}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "something went wrong")
        })?
        .into_iter()
        .map(|model| ResponseTaskData {
            id: model.id,
            title: model.title,
            description: model.description,
            priority: model.priority,
            completed_at: model
                .completed_at
                .map(|completed_at| completed_at.to_string()),
            // deleted_at: model.deleted_at,
            // is_default: model.is_default,
        })
        .collect::<Vec<ResponseTaskData>>();
    // //! turbo fish
    Ok(Json(ResponseTasksContainer { data: table_task }))
}
