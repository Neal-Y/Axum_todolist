mod routes;

use sea_orm::Database;

pub async fn run(database_uri: &str) {
    let database = Database::connect(database_uri)
        .await
        .expect("忘記docker-compose up YOU SUCH IDIOT!");
    let app = routes::create_route(database).await;

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
