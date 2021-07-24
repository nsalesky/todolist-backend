use rocket::http::Status;
use rocket::response::Debug;
use rocket::response::status;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use rocket_sync_db_pools::diesel;
use rocket_sync_db_pools::diesel::prelude::*;

use crate::auth::UserToken;
use crate::database::PostgresDbConn;
use crate::models::item::ItemDTO;
use crate::models::list::ListDTO;
use crate::models::response::Response;
use crate::services::list_service;

/// Attempts to create a new list
#[post("/lists", format = "json", data = "<new_list>")]
pub async fn create_list(new_list: Json<ListDTO>, token: UserToken, db: PostgresDbConn) -> status::Custom<Json<Response>> {
    let response = list_service::create_list(new_list.into_inner(), token.username, db).await;

    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}

/// Attempts to delete an existing list.
#[delete("/lists/<list_id>")]
pub async fn delete_list(list_id: i32, token: UserToken, db: PostgresDbConn) -> status::Custom<Json<Response>> {
    let response = list_service::delete_list(list_id, token.id, db).await;

    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}

/// Attempts to add an item to an existing list.
#[post("/lists/<list_id>/add", format = "json", data = "<new_item>")]
pub async fn post_item(list_id: i32, new_item: Json<ItemDTO>, token: UserToken, db: PostgresDbConn) -> status::Custom<Json<Response>> {
    let response =
        list_service::add_item_to_list(list_id, token.id, new_item.into_inner(), db).await;

    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}

/// Attempts to delete an item from a list.
#[delete("/lists/<list_id>/<item_id>")]
pub async fn delete_item(list_id: i32, item_id: i32, token: UserToken, db: PostgresDbConn) -> status::Custom<Json<Response>> {
    let response = list_service::delete_item(list_id, token.id, item_id, db).await;

    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}

/// Attempts to get the lists that the logged-in user can access.
#[get("/lists")]
pub async fn get_lists(token: UserToken, db: PostgresDbConn) -> status::Custom<Json<Response>> {
    let response = list_service::get_lists_for_user(token.id, db).await;

    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}

/// Attempts to get a full single list with items for the logged-in user.
#[get("/lists/<list_id>")]
pub async fn get_list(list_id: i32, token: UserToken, db: PostgresDbConn) -> status::Custom<Json<Response>> {
    let response = list_service::get_list(list_id, token.id, db).await;

    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}