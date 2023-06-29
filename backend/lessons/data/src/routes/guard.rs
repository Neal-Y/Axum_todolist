use axum::{
    extract::State,
    headers::{authorization::Bearer, Authorization},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    TypedHeader,
};
use sea_orm::{
 ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
};

use super::{
    database_source::users::{self, Entity as Users},
    utils::{app_error::AppError, jwt::verify_jwt},
};

pub async fn middleware_guard<T>(
    State(database_field): State<DatabaseConnection>,
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    mut request: Request<T>,
    next: Next<T>,
) -> Result<Response, AppError> {
    let request_token = 
        // request
        // .headers()
        // .typed_get::<Authorization<Bearer>>()
        // .ok_or_else(|| AppError::new(StatusCode::BAD_REQUEST, "plz try again"))?
        token.token().to_string();

    // let database_field = request
    //     .extensions()
    //     .get::<DatabaseConnection>()
    //     .ok_or_else(|| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "server's error"))?;

    let compare_user = Users::find()
        .filter(users::Column::Token.eq(Some(request_token.clone())))
        .one(&database_field)
        .await
        .map_err(|error| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, error.to_string()))?;

    verify_jwt(&request_token)?; //? set in button is because the some possibility about time attack

    let user = if let Some(user) = compare_user {
        user.into_active_model()
    } else {
        return Err(AppError::new(StatusCode::UNAUTHORIZED, "Invalid"));
    };

    request.extensions_mut().insert(user);
    Ok(next.run(request).await)
}
