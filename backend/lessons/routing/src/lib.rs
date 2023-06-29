use routes::crate_route;
mod routes;

pub async fn run() {
    let app = crate_route();

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}
