use serde_json::Value;
use rocket::response::status;
use rocket::serde::json::Json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub message: String,
    pub data: Value,
}

#[derive(Debug)]
pub struct ResponseWithStatus {
    pub status_code: u16,
    pub response: Response,
}

// pub type ResponseWithStatus = status::Custom<Json<Response>>;