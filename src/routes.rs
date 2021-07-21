use rocket::response::Debug;
use rocket::response::status;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use rocket_sync_db_pools::diesel;
use rocket_sync_db_pools::diesel::prelude::*;
use serde_json::json;

use crate::database::PostgresDbConn;
use crate::models::user::{User, UserDTO, LoginDTO};
use crate::schema::users;
use crate::models::response::{ResponseWithStatus, Response};
use rocket::http::Status;
use crate::constants;
use crate::jwt;

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
                    data: serde_json::to_value(
                        json!({
                            "token": jwt::generate_token(result),
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