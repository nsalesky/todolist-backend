use rocket::http::Status;
use serde_json::json;

use crate::auth;
use crate::constants;
use crate::database::PostgresDbConn;
use crate::models::response::{Response, ResponseWithStatus};
use crate::models::user::{LoginDTO, User, UserDTO};

/// Attempts to signup a new user with the specified user information.
/// If successful, informs the user that the account has been created successfully.
/// If unsuccessful, returns a failed response informing the user.
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
                    data: serde_json::to_value("").unwrap(),
                },
            }
        }
    }).await
}

/// Attempts to login the user with the specified login information.
/// If successful, returns that user's authentication token in the body of the response.
/// If unsuccessful, returns a failed response informing the user.
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
                },
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

// todo: I added this function to debug the authentication tokens, remember to remove it later
/// Attempts to get the information for the user with the specified username.
/// If successful, returns that user's information in the response.
/// If unsuccessful, returns a failed response informing that no such user exists.
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
