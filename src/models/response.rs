use rocket::response::status;
use rocket::serde::json::Json;
use serde_json::Value;

/// A response containing a header message and some JSON data.
#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub message: String,
    pub data: Value,
}

/// Wraps a response with a given status code.
#[derive(Debug)]
pub struct ResponseWithStatus {
    pub status_code: u16,
    pub response: Response,
}