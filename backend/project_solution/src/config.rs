use dotenvy::dotenv;
use dotenvy_macro::dotenv;

pub fn sever_config() -> (String, String) {
    dotenv().ok();

    // //? 因為dotenv!()是在抓compile time 抓參數沒抓到就報錯，所以根本不需要使用run time 的錯誤處理去額外加上，畢竟之所以要自定義的錯誤處理是擔心run time crash
    // let database_uri = dotenv::var("DATABASE_URL").map_err(|_| {
    //     AppError::new(
    //         StatusCode::INTERNAL_SERVER_ERROR,
    //         "Internal sever error, plz try again later",
    //     )
    // })?;
    // let jwt_secret = dotenv::var("JWT_SECRET").map_err(|_| {
    //     AppError::new(
    //         StatusCode::INTERNAL_SERVER_ERROR,
    //         "Internal sever error, plz try again later",
    //     )
    // })?;

    let database_uri = dotenv!("DATABASE_URL").to_string();
    let jwt_secret = dotenv!("JWT_SECRET").to_string();
    (database_uri, jwt_secret)
}

//
