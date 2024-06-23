use serde::Serialize;

/// ヘルスチェックAPI用のレスポンス
#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
}
