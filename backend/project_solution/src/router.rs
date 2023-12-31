use crate::{
    middleware::request_authentication::get_authorization_from_request,
    routes::{
        hello_world::hello_world,
        tasks::{
            create_task::new_task,
            delete_task::delete_task,
            get_all_tasks::get_tasks,
            get_one_task::get_task,
            update_tasks::{mark_completed, mark_uncompleted, update_all_field_with_task},
        },
        users::{create_user::create_user, login::user_login, logout::user_logout},
    },
    utilities::app_state::AppState,
};
use axum::{
    middleware,
    routing::{delete, get, patch, post, put},
    Router,
};

pub async fn crate_route(app_state: AppState) -> Router {
    Router::new()
        .route("/api/v1/users/logout", post(user_logout))
        .route("/api/v1/tasks", post(new_task))
        .route("/api/v1/tasks", get(get_tasks))
        .route("/api/v1/tasks/:task_id", get(get_task))
        .route("/api/v1/tasks/:task_id/completed", put(mark_completed))
        .route("/api/v1/tasks/:task_id/uncompleted", put(mark_uncompleted))
        .route("/api/v1/tasks/:task_id", patch(update_all_field_with_task))
        .route("/api/v1/tasks/:task_id", delete(delete_task))
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            get_authorization_from_request,
        ))
        .route("/", get(hello_world))
        .route("/api/v1/users", post(create_user))
        .route("/api/v1/users/login", post(user_login))
        .with_state(app_state)
}
