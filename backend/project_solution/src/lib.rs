use axum::http::StatusCode;
use router::crate_route;
use std::net::SocketAddr;
use utilities::{app_error::AppError, app_state::AppState};

// 這裡使用mod是因為這是lib.rs，之前之所以不用是因為我在lib.rs直接mod routes
// 然後把下面這些全部mod 在那裡
pub mod config;
mod database;
mod middleware;
mod router;
mod routes;
pub mod utilities;

pub async fn run(app_state: AppState) -> Result<(), AppError> {
    let app = crate_route(app_state).await;

    axum::Server::bind(&SocketAddr::from(([127, 0, 0, 1], 3000)))
        .serve(app.into_make_service())
        .await
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "I can run MDFK"))?;

    Ok(())
}
