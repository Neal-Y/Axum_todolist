use axum::{
    extract::{FromRef, State},
    Extension,
};

#[derive(Clone, FromRef)]
pub struct SharedData {
    pub message: String,
}

pub async fn middleware_message(State(share_data): State<String>) -> String {
    share_data
}

// 這Extension(share_data)是一種模式匹配(pattern matching)，他會自行解構成Extension如果為None會直接報錯
// pub async fn middleware_message(Extension(share_data): Extension<SharedData>) -> String {
//     share_data.message.to_string()
// }

// 而這種則不會，我們需要自行定義當傳進來的不是Extension時
// pub async fn middleware_message(share_data: Extension<SharedData>) -> String {
//     share_data.message.to_string()
// }

/*

有幾種情況可能不會回傳 Extension

當你的 middleware 不使用 Extension，或者該請求沒有觸發使用 Extension 的 middleware 時。

例如，如果你的 middleware 僅在請求的某些條件下（如特定的路由或 HTTP 方法）設置 Extension，那麼不滿足這些條件的請求將不會有 Extension。

如果先前的 middleware 遇到錯誤並提前結束了響應，那麼設置 Extension 的 middleware 可能永遠不會被執行。

如果你的 handler 並不期望每個請求都有一個 Extension，那麼在某些請求中可能不會提供 Extension。

例如，某些請求可能僅用於健康檢查或監控，而不需要訪問 Extension 中的數據。

這些情況下，如果你的 handler 強制期待一個 Extension，則可能會導致錯誤或panic。

//?因此，你可能需要在你的 handler 中處理 Extension 為 None 的情況，或者確保你的 middleware 在每次請求中都設置了 Extension。

*/

#[derive(Clone)]
pub struct HeaderMessage(pub String);

pub async fn read_custom_middleware(Extension(head_msg): Extension<HeaderMessage>) -> String {
    head_msg.0
}
