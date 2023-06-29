use axum::http::StatusCode;
use project_solution::{
    config::sever_config,
    run,
    utilities::{app_error::AppError, app_state::AppState, token_wrapper::TokenWrapper},
};
use sea_orm::Database;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let (database_url, jwt_secret) = sever_config();

    let database = Database::connect(database_url).await.map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "database can't connected",
        )
    })?;

    let app_state = AppState {
        database,
        jwt_secret: TokenWrapper(jwt_secret),
    };

    run(app_state).await?;
    Ok(())
}
// 不用放在main
