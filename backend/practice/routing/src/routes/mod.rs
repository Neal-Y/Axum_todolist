mod custom_response;
mod error_state;
mod header;
mod json;
mod middleware;
mod path_variable;
mod query_params;
mod set_custom_middleware;
mod string_type;

use axum::{
    http::Method,
    middleware::from_fn,
    routing::{get, post},
    Router,
};

use custom_response::custom_response;
use error_state::i_am_teapot;
use header::{header_map, header_type};
use json::{mirror_json, return_json};
use middleware::{middleware_message, read_custom_middleware, SharedData};
use path_variable::path_variable;
use query_params::query_params;
use set_custom_middleware::set_custom_middleware;
use string_type::{hello, mirror_string};
use tower_http::cors::{Any, CorsLayer};

pub fn crate_route() -> Router {
    let cors = CorsLayer::new() //? Cross-Origin Resource Sharing
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let share_data = SharedData {
        message: "hello, this is Extension middleware".to_string(),
    };

    Router::new()
        .route("/custom_middleware", get(read_custom_middleware))
        .route_layer(from_fn(set_custom_middleware))
        .route("/hello", post(hello))
        .route("/mirror", post(mirror_string))
        .route("/json", post(mirror_json))
        .route("/path_test/:id", get(path_variable))
        .route("/query_test", get(query_params))
        .route("/header_type", get(header_type))
        .layer(cors)
        .route("/header_map", get(header_map))
        .route("/middleware", get(middleware_message))
        // .layer(Extension(share_data))
        //? 如果你想在所有路由中共享一些數據，並且該數據可以克隆（即實現了Clone trait），
        //? 那麼你可以使用 .with_state(...)。
        //? 此方法在每個請求時克隆該數據並將其存儲在 Axum 層中，
        //? 可以在你的處理器中使用 extract::Extension 或 extract::State 來訪問它。
        .with_state(share_data)
        .route("/failure_response", get(i_am_teapot))
        .route("/custom_response", post(custom_response))
        .route("/return_json", post(return_json))
}
