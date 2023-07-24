mod create_task;
mod custom_json_extractor;
pub mod database_source; //? re-exporting需要明確的一層一層的轉出，我不能/src/lib.rs中直接呼叫/src/routes/database_source/mod.rs
mod delete_tasks;
mod get_tasks;
mod guard;
mod hello_world;
mod partial_update_tasks;
mod update_tasks;
mod user;
pub mod utils;
mod validate_json;

use axum::{
    extract::FromRef,
    middleware,
    routing::{delete, get, patch, post, put},
    Router,
};

use create_task::new_task;
use custom_json_extractor::json_extractor;
use delete_tasks::delete_tasks;
use get_tasks::{get_task, get_tasks};
use guard::middleware_guard;
use hello_world::hello;
use partial_update_tasks::partial_update;
use sea_orm::DatabaseConnection;
use update_tasks::atomic_update;
use user::{login, logout, new_user};
use validate_json::validate_data;

#[derive(Clone, FromRef)]
pub struct AppState {
    database: DatabaseConnection,
}

pub async fn create_route(database: DatabaseConnection) -> Router {
    let app_state = AppState { database };
    Router::new()
        .route("/users/logout", post(logout))
        .route("/hello", get(hello))
        // .route_layer(middleware::from_fn(middleware_guard))
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            middleware_guard,
        ))
        .route("/validate_data", post(validate_data))
        .route("/custom_json_extractor", post(json_extractor))
        .route("/crate_task", post(new_task))
        .route("/get_single_task/:task_id", get(get_task))
        .route("/get_all_tasks", get(get_tasks))
        .route("/update_task/:task_id", put(atomic_update))
        .route("/partial_update_task/:task_id", patch(partial_update))
        .route("/delete_tasks/:task_id", delete(delete_tasks))
        .route("/create_user", post(new_user))
        .route("/users/login", post(login))
        // .layer(Extension(database))
        .with_state(app_state)
}
