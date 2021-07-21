use rocket::http::Status;
use rocket::response::Debug;
use rocket::response::status;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use rocket_sync_db_pools::diesel;
use rocket_sync_db_pools::diesel::prelude::*;

use crate::auth;
use crate::auth::UserToken;
use crate::constants;
use crate::database::PostgresDbConn;
use crate::models::response::{Response, ResponseWithStatus};
use crate::models::user::{LoginDTO, User, UserDTO};
use crate::schema::users;
use crate::services::account_service;

// type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

// #[post("/users/create", format = "json", data = "<create_user>")]
// pub async fn create_user(db: PostgresDbConn, create_user: Json<CreateUserRequest>) -> Result<Created<Json<&'static str>>> {
//     let user = UserDTO::from_create(create_user.into_inner());
//
//     db.run(move |conn| {
//         diesel::insert_into(users::table)
//             .values(user)
//             .execute(conn)
//     }).await?;
//
//     Ok(Created::new("/").body(Json("hello there")))
// }
//
// #[get("/users/<id>")]
// pub async fn get_user(db: PostgresDbConn, id: i32) -> Option<Json<User>> {
//     db.run(move |conn| {
//         users::table
//             .filter(users::id.eq(id))
//             .first(conn)
//     }).await.map(Json).ok()
// }

#[post("/signup", format = "json", data = "<user>")]
pub async fn signup(user: Json<UserDTO>, db: PostgresDbConn) -> status::Custom<Json<Response>> {
    let response = account_service::signup(user.into_inner(), db).await;

    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}

#[post("/login", format = "json", data = "<login>")]
pub async fn login(login: Json<LoginDTO>, db: PostgresDbConn) -> status::Custom<Json<Response>> {
    let response = account_service::login(login.into_inner(), db).await;

    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}

#[get("/users")]
pub async fn get_user(token: UserToken, db: PostgresDbConn) -> status::Custom<Json<Response>> {
    let response = account_service::get_user(token.username, db).await;

    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}