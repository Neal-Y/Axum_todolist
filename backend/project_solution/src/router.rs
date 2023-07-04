use crate::{
    middleware::request_authentication::get_authorization_from_request,
    routes::{
        hello_world::hello_world,
        tasks::create_task::new_task,
        users::{create_user::create_user, login::user_login, logout::user_logout},
    },
    utilities::app_state::AppState,
};
use axum::{
    middleware,
    routing::{get, post},
    Router,
};

pub async fn crate_route(app_state: AppState) -> Router {
    Router::new()
        .route("/api/v1/users/logout", post(user_logout))
        .route("/api/v1/tasks", post(new_task))
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            get_authorization_from_request,
        ))
        .route("/", get(hello_world))
        .route("/api/v1/users", post(create_user))
        .route("/api/v1/users/login", post(user_login))
        .with_state(app_state)
}
