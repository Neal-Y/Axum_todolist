use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct QueryStruct {
    message: String,
    id: i32,
}

pub async fn query_params(Query(query): Query<QueryStruct>) -> Json<QueryStruct> {
    Json(query)
}
