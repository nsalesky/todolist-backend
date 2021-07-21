use crate::models::response::{ResponseWithStatus, Response};
use crate::database::PostgresDbConn;
use crate::models::user::{User, UserDTO, LoginDTO};
use rocket::http::Status;
use crate::constants;
use crate::auth;
use serde_json::json;

pub async fn signup(user: UserDTO, db: PostgresDbConn) -> ResponseWithStatus {
    db.run(|conn| {
        if User::signup(user, &conn) {
            ResponseWithStatus {
                status_code: Status::Ok.code,
                response: Response {
                    message: String::from(constants::MESSAGE_SIGNUP_SUCCESS),
                    data: serde_json::to_value("").unwrap(),
                },
            }
        } else {
            ResponseWithStatus {
                status_code: Status::BadRequest.code,
                response: Response {
                    message: String::from(constants::MESSAGE_SIGNUP_FAILED),
                    data: serde_json::to_value("").unwrap()
                }
            }
        }

    }).await
}

pub async fn login(login: LoginDTO, db: PostgresDbConn) -> ResponseWithStatus {
    db.run(|conn| {
        if let Some(result) = User::login(login, &conn) {
            ResponseWithStatus {
                status_code: Status::Ok.code,
                response: Response {
                    message: String::from(constants::MESSAGE_LOGIN_SUCCESS),
                    data: serde_json::to_value(json!({
                        "token": auth::generate_token(result),
                        "type": "Bearer"
                    })).unwrap(),
                }
            }
        } else {
            ResponseWithStatus {
                status_code: Status::BadRequest.code,
                response: Response {
                    message: String::from(constants::MESSAGE_LOGIN_FAILED),
                    data: serde_json::to_value("").unwrap(),
                },
            }
        }
    }).await
}

pub async fn get_user(username: String, db: PostgresDbConn) -> ResponseWithStatus {
    db.run(move |conn| {
        if let Some(user) = User::find_user_by_username(username.as_str(), conn) {
            ResponseWithStatus {
                status_code: Status::Ok.code,
                response: Response {
                    message: String::from(constants::MESSAGE_GET_USER_SUCCESS),
                    data: serde_json::to_value(user).unwrap(),
                },
            }
        } else {
            ResponseWithStatus {
                status_code: Status::BadRequest.code,
                response: Response {
                    message: String::from(constants::MESSAGE_GET_USER_FAILED),
                    data: serde_json::to_value("").unwrap(),
                },
            }
        }
    }).await
}
