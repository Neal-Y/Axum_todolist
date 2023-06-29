use super::{
    database_source::users::{self, ActiveModel},
    utils::jwt::create_jwt,
};
use crate::routes::database_source::users::Entity as Users;
use axum::{extract::State, http::StatusCode, Extension, Json};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RequestUser {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct ResponseUser {
    username: String,
    id: i32,
    token: String,
}

pub async fn new_user(
    State(database): State<DatabaseConnection>,
    Json(user_info): Json<RequestUser>, //? 這會消耗request中的body所以必須排後面 button down
) -> Result<Json<ResponseUser>, StatusCode> {
    let jwt = create_jwt()?;
    let new_user = users::ActiveModel {
        username: Set(user_info.username),
        password: Set(hash_password(user_info.password)?),
        token: Set(Some(jwt)),
        ..Default::default()
    }
    .save(&database)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ResponseUser {
        username: new_user.username.unwrap(),
        id: new_user.id.unwrap(),
        token: new_user.token.unwrap().unwrap(),
    }))
}

pub async fn login(
    State(database): State<DatabaseConnection>,
    Json(user_info): Json<RequestUser>, //? 這會消耗request中的body所以必須排後面 button down
) -> Result<Json<ResponseUser>, StatusCode> {
    let database_user = Users::find()
        .filter(users::Column::Username.eq(user_info.username))
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(database_user) = database_user {
        if !verify_password(user_info.password, &database_user.password)? {
            //? "?"可以提取Ok()分支，因為function是回傳一個Result<bool, StatusCode>不是單純bool，所以前面"!"這樣有問題
            return Err(StatusCode::UNAUTHORIZED);
        }

        let new_token = create_jwt()?;
        let mut updated_user = database_user.into_active_model();

        updated_user.token = Set(Some(new_token));

        let saved_user = updated_user
            .save(&database)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json(ResponseUser {
            username: saved_user.username.unwrap(),
            id: saved_user.id.unwrap(),
            token: saved_user.token.unwrap().unwrap(),
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn logout(
    Extension(mut user): Extension<ActiveModel>,
    State(database): State<DatabaseConnection>,
) -> Result<(), StatusCode> {
    user.token = Set(None);

    user.save(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}

fn hash_password(password: String) -> Result<String, StatusCode> {
    bcrypt::hash(password, 14).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

fn verify_password(password: String, hash: &str) -> Result<bool, StatusCode> {
    bcrypt::verify(password, hash).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
