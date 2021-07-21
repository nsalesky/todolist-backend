use rocket::http::Status;
use rocket::response::Debug;
use rocket::response::status;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use rocket_sync_db_pools::diesel;
use rocket_sync_db_pools::diesel::prelude::*;
use crate::auth::UserToken;
use crate::database::PostgresDbConn;
use crate::models::response::Response;
use crate::models::list::ListDTO;
use crate::services::list_service;

#[post("/lists", format = "json", data = "<new_list>")]
pub async fn create_list(new_list: Json<ListDTO>, token: UserToken, db: PostgresDbConn) -> status::Custom<Json<Response>> {
    let response = list_service::create_list(new_list.into_inner(), token.username, db).await;

    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}