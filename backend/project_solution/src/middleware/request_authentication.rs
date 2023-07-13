use axum::{
    extract::State,
    http::{HeaderMap, Request, StatusCode},
    middleware::Next,
    response::Response,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{
    database::users::{self, Entity as Users},
    utilities::{app_error::AppError, app_state::AppState, jwt::verify_jwt_token},
};

pub async fn get_authorization_from_request<T>(
    headers: HeaderMap,
    State(app_state): State<AppState>,
    mut request: Request<T>,
    next: Next<T>,
) -> Result<Response, AppError> {
    let squeezed_header_token = headers
        .get("x-auth-token")
        .ok_or_else(|| AppError::new(StatusCode::UNAUTHORIZED, "not authenticated!"))?
        .to_str()
        .map_err(|error| {
            eprintln!("Invalid x-auth-token{:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "token")
        })?;

    // 我們這行確認request body 中的token是否有效，有就是說如果可以我們再進行下一步對資料庫user是否有這token的確認
    verify_jwt_token(&app_state.jwt_secret.0, squeezed_header_token)?;

    // 從database 裡面找出是哪個user的token
    let checked_token_from_user = Users::find()
        .filter(users::Column::Token.eq(squeezed_header_token))
        .one(&app_state.database)
        .await
        .map_err(|error| {
            eprintln!("{:?}", error);
            AppError::new(StatusCode::NOT_FOUND, "did't find database user")
        })?
        .ok_or_else(|| AppError::new(StatusCode::UNAUTHORIZED, "try again"))?;

    //? 不需要，因為如果找不到用戶， 『one』 應該會返回一個錯誤，它將立即返回一個錯誤
    // 畢竟token是option 檢查到底有沒有
    // if let Some(checked_token_from_user) = checked_token_from_user {
    //     request.extensions_mut().insert(checked_token_from_user);
    // } else {
    //     return Err(AppError::new(StatusCode::UNAUTHORIZED, "try again"));
    // }

    request.extensions_mut().insert(checked_token_from_user);
    Ok(next.run(request).await)
}

/*
我來一步步講解一下我剛剛代碼的思路
    let squeezed_header_token = if let Some(token) = headers.get("x-auth-token") {
        token.to_str().map_err(|error| {
            eprintln!("Invalid x-auth-token{:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "token")
        })?
    } else {
        return Err(AppError::new(StatusCode::UNAUTHORIZED, "Invalid"));
    };
這段是在講說我想從我吃進來的參數request中萃取出“x-auth-token”的值並且對他進行一些錯誤處理，取出來後我放進這個叫做squeezed_header_token的變數當中。

接著我用verify_jwt_token()這函數確認squeezed_header_token是否有效是否是當時給user的jwt token
    verify_jwt_token(&app_state.jwt_secret.0, squeezed_header_token)?;

並且對database 裡面找出是哪個user的token
    let squeezed_database_user = Users::find()
        .filter(users::Column::Token.eq(squeezed_header_token))
        .one(&app_state.database)
        .await
        .map_err(|error| {
            eprintln!("{:?}", error);
            AppError::new(StatusCode::NOT_FOUND, "did't find database user")
        })?;

最後畢竟資料庫的token是設定成Option 需要檢查到底有沒有
    if let Some(checked_token_from_user) = squeezed_database_user {
        request.extensions_mut().insert(checked_token_from_user);
    } else {
        return Err(AppError::new(StatusCode::UNAUTHORIZED, "try again"));
    }

    Ok(next.run(request).await)
}

我的問題是我的檢查是不是某種程度上重複了？
從一開始從request提出token->再來用verify_jwt_token()確認是不是我發放的token->接著檢查是database中哪個user的token
那在這個階段如果通過不就代表我不需要去檢查是否為none不是嗎？就不用最後那段
if let Some(checked_token_from_user) = squeezed_database_user {
        request.extensions_mut().insert(checked_token_from_user);
    } else {
        return Err(AppError::new(StatusCode::UNAUTHORIZED, "try again"));
    }
 */
