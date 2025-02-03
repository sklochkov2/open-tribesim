use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseError {
    pub code: u32,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RunSimResponse {
    pub status: String,
    pub error: ResponseError,
    pub sim_uuid: String,
}

impl ResponseError {
    pub fn ok() -> Self {
        Self {
            code: 200,
            message: String::new(),
        }
    }
}
