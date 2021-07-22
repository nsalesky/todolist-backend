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

impl ResponseWithStatus {
    /// Creates a `ResponseWithStatus` with the given status code and message, and empty data
    pub fn with(status_code: u16, message: &str) -> ResponseWithStatus {
        ResponseWithStatus {
            status_code,
            response: Response {
                message: String::from(message),
                data: serde_json::to_value("").unwrap(),
            },
        }
    }
}