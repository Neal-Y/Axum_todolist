use axum::{headers::UserAgent, http::HeaderMap, TypedHeader};

// 這是對於header中 單一的並且已被定義常見的資料去下手(Auth Token、User-Agent等)，而如果是想要使用“自定義header”傳到後台，就需要header_map
pub async fn header_type(TypedHeader(user_agent): TypedHeader<UserAgent>) -> String {
    user_agent.to_string()
}

// 針對沒有被TypeHeader定義的header資料就是使用HeaderMap，他會把所有資料抓下來變成一個集合
pub async fn header_map(header: HeaderMap) -> String {
    header
        .get("x-message")
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned()
}
