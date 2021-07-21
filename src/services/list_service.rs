use crate::models::user::{User, UserDTO};
use crate::database::PostgresDbConn;
use crate::models::response::{ResponseWithStatus, Response};
use crate::models::list::{List, ListDTO};
use rocket::http::Status;
use crate::constants;
use crate::models::user_lists::UserList;

/// Attempts to create the list with the specified information and associate it with its owner.
/// Responds with the appropriate status.
pub async fn create_list(list: ListDTO, owner: UserDTO, db: PostgresDbConn) -> ResponseWithStatus {
    db.run(move |conn| {
        if let Some(user) = User::find_user_by_username(&owner.username, conn) {
            if let Some(result_list) = List::create_list(list, conn) {
                if UserList::associate_list(result_list, user, conn) {
                    ResponseWithStatus {
                        status_code: Status::Ok.code,
                        response: Response {
                            message: String::from(constants::MESSAGE_CREATE_LIST_SUCCESS),
                            data: serde_json::to_value("").unwrap(),
                        }
                    }
                } else {
                    ResponseWithStatus {
                        status_code: Status::BadRequest.code,
                        response: Response {
                            message: String::from(constants::MESSAGE_CREATE_LIST_ASSOCIATE_FAIL),
                            data: serde_json::to_value("").unwrap(),
                        }
                    }
                }
            } else {
                ResponseWithStatus {
                    status_code: Status::BadRequest.code,
                    response: Response {
                        message: String::from(constants::MESSAGE_CREATE_LIST_FAILED),
                        data: serde_json::to_value("").unwrap(),
                    }
                }
            }
        } else {
            ResponseWithStatus {
                status_code: Status::BadRequest.code,
                response: Response {
                    message: String::from(constants::MESSAGE_USER_NOT_FOUND),
                    data: serde_json::to_value("").unwrap(),
                },
            }
        }


    }).await
}