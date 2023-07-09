use axum::{extract::State, http::StatusCode, Extension, Json};
use sea_orm::{prelude::DateTimeWithTimeZone, ColumnTrait, EntityTrait, QueryFilter};
use serde::Serialize;

use crate::{
    database::{
        tasks::{self, Entity as Tasks},
        users::{self, Entity as Users, Model as UserModel},
    },
    utilities::{app_error::AppError, app_state::AppState},
};

use super::ResponseTasksContainer;

pub async fn get_tasks(
    State(app_state): State<AppState>,
    Extension(user): Extension<UserModel>,
) -> Result<Json<Vec<ResponseTasksContainer>>, AppError> {
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
        })?;

    todo!()
}
