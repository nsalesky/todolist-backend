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
use crate::models::user::{LoginDTO, User, UserDTO, UpdatePreferredName, UpdatePassword};
use crate::schema::users;
use crate::services::account_service;

/// A route to sign up a new user with the specified JSON user information. Attempts to
/// insert them into the users table, and returns a response indicating whether it was successful
/// or not.
#[post("/signup", format = "json", data = "<user>")]
pub async fn signup(user: Json<UserDTO>, db: PostgresDbConn) -> status::Custom<Json<Response>> {
    let response = account_service::signup(user.into_inner(), db).await;

    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}

/// A route to log in a user with the given JSON login information. If successful, includes their
/// authentication token in the response. Otherwise, indicates the failure through the status code.
#[post("/login", format = "json", data = "<login>")]
pub async fn login(login: Json<LoginDTO>, db: PostgresDbConn) -> status::Custom<Json<Response>> {
    let response = account_service::login(login.into_inner(), db).await;

    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}

/// A route to get a user's basic information.
#[get("/users")]
pub async fn get_user(token: UserToken, db: PostgresDbConn) -> status::Custom<Json<Response>> {
    let response = account_service::get_user(token.username, db).await;

    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}

/// A route to update a user's preferred name to the new value in `preferred_name`.
#[put("/users/name", format = "json", data = "<preferred_name>")]
pub async fn put_preferred_name(preferred_name: Json<UpdatePreferredName>, token: UserToken, db: PostgresDbConn) -> status::Custom<Json<Response>> {
    let response = account_service::put_preferred_name(token.username, preferred_name.into_inner(), db).await;

    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response)
    )
}

/// A route to update a user's password to the new value in `password`.
#[put("/users/password", format = "json", data = "<password>")]
pub async fn put_password(password: Json<UpdatePassword>, token: UserToken, db: PostgresDbConn) -> status::Custom<Json<Response>> {
    let response = account_service::put_password(token.username, password.into_inner(), db).await;

    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response)
    )
}