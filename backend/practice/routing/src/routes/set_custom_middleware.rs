use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

use middleware::HeaderMessage;

use super::middleware;

pub async fn set_custom_middleware<B>(
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let message = req
        .headers()
        .get("User-Agent")
        .ok_or_else(|| StatusCode::BAD_REQUEST)?
        .to_str()
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .to_owned();

    req.extensions_mut().insert(HeaderMessage(message));
    Ok(next.run(req).await)
}
