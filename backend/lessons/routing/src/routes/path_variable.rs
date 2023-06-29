use axum::extract::Path;

pub async fn path_variable(Path(id): Path<i32>) -> String {
    format!("this is your id {}", id)
}
