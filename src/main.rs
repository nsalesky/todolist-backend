extern crate crypto;
#[macro_use]
extern crate rocket;

mod user;
mod database;
mod model;
mod routes;

use rocket::{Build, Rocket, State};
use rocket::serde::json::{Json, Value};
use rocket_sync_db_pools::Connection;
use rocket_sync_db_pools::postgres::Client;
use rocket_sync_db_pools::postgres::Error;
use serde_json::json;

use crate::database::PostgresDbConn;
use crate::user::{CreateUser, hash_password};
use crate::model::User;


#[get("/")]
fn index() -> &'static str {
    "Hello world!"
}

#[post("/users", format = "json", data = "<create_user>")]
async fn create_user(conn: PostgresDbConn, create_user: Json<CreateUser>) -> Value {
    let create_user = create_user.into_inner();
    let password_hash = hash_password(&create_user.password);

//
//     json!({ "status": "ok", "id": id })
    json!({ "status": "ok" })
}

// #[get("/users")]
// async fn users_list(conn: PostgresDbConn) -> String {
//     let rows = database::get_users(conn).await;
//
//     let mut str = String::new();
//
//     for row in rows {
//         let user_id: i32 = row.get("user_id");
//         let username: String = row.get("username");
//         let name: String = row.get("name");
//         let email: String = row.get("email");
//
//         str.push_str(format!("[{}]: username = {}, email = {}, name = {}\n", user_id, username, email, name).as_str());
//     }
//
//     str
// }

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![index, create_user, routes::get_user])
        .attach(PostgresDbConn::fairing())
}