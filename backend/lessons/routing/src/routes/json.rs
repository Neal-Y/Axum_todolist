use axum::Json;
use serde::{Deserialize, Serialize};

// mirror_json 把client端的request body裡的json抓出來，然後塞到MirrorJson，接著利用SeverSideResponse包在response裡面回傳給client端
#[derive(Deserialize, Serialize, Debug)]
pub struct MirrorJson {
    message: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SeverSideResponse {
    message: String,
    seversidestirng: String,
}

pub async fn mirror_json(Json(body): Json<MirrorJson>) -> Json<SeverSideResponse> {
    Json(SeverSideResponse {
        message: body.message,
        seversidestirng: "hello, this is severside response".to_owned(),
    })
}

// 這邊比較像是回傳一個SeverSide創好的Json

#[derive(Serialize)]
pub struct Data {
    username: String,
    id: i32,
    message: String,
}

pub async fn return_json() -> Json<Data> {
    Json(Data {
        username: "neal".to_owned(),
        id: 6,
        message: "hello, this is neal".to_owned(),
    })
}
