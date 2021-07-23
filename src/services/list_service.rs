use rocket::http::Status;

use crate::constants;
use crate::database::PostgresDbConn;
use crate::models::item::{Item, ItemDTO};
use crate::models::list::{List, ListDTO};
use crate::models::response::{Response, ResponseWithStatus};
use crate::models::user::{User, UserDTO};
use crate::models::user_lists::UserList;

/// Attempts to create the list with the specified information and associate it with its owner.
/// Responds with the appropriate status.
pub async fn create_list(list: ListDTO, owner_username: String, db: PostgresDbConn) -> ResponseWithStatus {
    db.run(move |conn| {
        if let Some(user) = User::find_user_by_username(&owner_username, conn) {
            if let Some(result_list) = List::create_list(list, conn) {
                if UserList::associate_list(result_list, user, true, conn) {
                    ResponseWithStatus {
                        status_code: Status::Ok.code,
                        response: Response {
                            message: String::from(constants::MESSAGE_CREATE_LIST_SUCCESS),
                            data: serde_json::to_value("").unwrap(),
                        },
                    }
                } else {
                    ResponseWithStatus {
                        status_code: Status::BadRequest.code,
                        response: Response {
                            message: String::from(constants::MESSAGE_CREATE_LIST_ASSOCIATE_FAIL),
                            data: serde_json::to_value("").unwrap(),
                        },
                    }
                }
            } else {
                ResponseWithStatus {
                    status_code: Status::BadRequest.code,
                    response: Response {
                        message: String::from(constants::MESSAGE_CREATE_LIST_FAILED),
                        data: serde_json::to_value("").unwrap(),
                    },
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

/// Attempts to delete the list with the given `list_id`. Makes sure that the user with `user_id`
/// owns the list before deleting it. Also, through cascading, deletes any `UserList` or `Item`s
/// related to the list.
pub async fn delete_list(list_id: i32, user_id: i32, db: PostgresDbConn) -> ResponseWithStatus {
    db.run(move |conn| {
        if UserList::is_list_owner(list_id, user_id, conn) {
            if List::delete_list(list_id, conn) {
                ResponseWithStatus::with(Status::Ok.code, constants::MESSAGE_DELETE_LIST_SUCCESS)
            } else {
                ResponseWithStatus::with(Status::BadRequest.code, constants::MESSAGE_DELETE_LIST_FAILED)
            }
        } else {
            ResponseWithStatus::with(Status::BadRequest.code, constants::MESSAGE_DELETE_NOT_OWNER)
        }
    }).await
}

/// Attempts to add the given `item` to the list with the specified `list_id`. Responds with the
/// appropriate status. Makes sure that the user with `user_id` has access to that list.
pub async fn add_item_to_list(list_id: i32, user_id: i32, item: ItemDTO, db: PostgresDbConn) -> ResponseWithStatus {
    db.run(move |conn| {
        if !UserList::has_list_access(list_id, user_id, conn) {
            return ResponseWithStatus::with(Status::BadRequest.code, constants::MESSAGE_NO_ACCESS);
        }

        if Item::create_item_for_list(item, list_id, conn) {
            ResponseWithStatus::with(Status::Ok.code, constants::MESSAGE_CREATE_ITEM_SUCCESS)
        } else {
            ResponseWithStatus::with(Status::BadRequest.code, constants::MESSAGE_CREATE_ITEM_FAILED)
        }
    }).await
}

/// Attempts to delete the item with the given `item_id`. First, ensures that the user has access to the containing list
pub async fn delete_item(list_id: i32, user_id: i32, item_id: i32, db: PostgresDbConn) -> ResponseWithStatus {
    db.run(move |conn| {
        if !UserList::has_list_access(list_id, user_id, conn) {
            return ResponseWithStatus::with(Status::BadRequest.code, constants::MESSAGE_NO_ACCESS);
        }

        if Item::owned_by_list(item_id, list_id, conn)
            && Item::delete_item(item_id, conn) {
            ResponseWithStatus::with(Status::Ok.code, constants::MESSAGE_DELETE_ITEM_SUCCESS)
        } else {
            ResponseWithStatus::with(Status::BadRequest.code, constants::MESSAGE_DELETE_ITEM_FAILED)
        }
    }).await
}

/// Responds with a JSON object containing basic information about each list that the given user has access to.
pub async fn get_lists_for_user(user_id: i32, db: PostgresDbConn) -> ResponseWithStatus {
    db.run(move |conn| {
        if let Some(lists) = List::find_lists_for_user(user_id, conn) {
            ResponseWithStatus {
                status_code: Status::Ok.code,
                response: Response {
                    message: String::from(constants::MESSAGE_OK),
                    data: serde_json::to_value(lists).unwrap(),
                },
            }


        } else {
            ResponseWithStatus::with(Status::BadRequest.code, constants::MESSAGE_USER_NOT_FOUND)
        }
    }).await
}