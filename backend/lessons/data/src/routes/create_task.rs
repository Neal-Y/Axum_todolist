use super::database_source::tasks;
use super::database_source::users::{self, Entity as Users};
use axum::extract::State;
use axum::{
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    Json, TypedHeader,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestTask {
    priority: Option<String>,
    title: String,
    description: Option<String>,
}

pub async fn new_task(
    State(database): State<DatabaseConnection>,
    authorization: TypedHeader<Authorization<Bearer>>,
    Json(task_info): Json<RequestTask>,
) -> Result<(), StatusCode> {
    let token = authorization.token(); //? get the request token, and compare with the database token

    let user_token_from_database = if let Some(user) = Users::find()
        .filter(users::Column::Token.eq(token))
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        user
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let new_task = tasks::ActiveModel {
        priority: Set(task_info.priority),
        title: Set(task_info.title),
        description: Set(task_info.description),
        user_id: Set(Some(user_token_from_database.id)),
        ..Default::default()
    };

    new_task.save(&database).await.unwrap();

    Ok(())
}
/*
   首先ActiveModel是SeaORM對於database的操作
   在所有不是“必要”的項目中也就是使用Option的，在進行操作都要用Some()包裹
   Set 是 SeaORM 的一種特定結構，它用來表示一個要設置的欄位值
   ..Default::default()中".."是將剩下的對其他欄位透過Default::default()自動填值
   save 就是將資料存進去 ->存到哪呢？存到&database 也就是傳進來的DatabaseConnection //! 是引用
*/
