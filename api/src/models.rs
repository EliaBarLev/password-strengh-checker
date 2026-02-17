use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ScoreRequest {
    pub password: String,
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
}
