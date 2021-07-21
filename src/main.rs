#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;

use rocket::{Build, Rocket, State};
use rocket::serde::json::{Json, Value};
use rocket_sync_db_pools::Connection;
use serde_json::json;

use crate::database::PostgresDbConn;

mod database;
mod models;
mod routes;
mod schema;
mod auth;
mod constants;
mod services;

#[get("/")]
fn index() -> &'static str {
    "Hello world!"
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![index, routes::signup, routes::login, routes::get_user])
        .attach(PostgresDbConn::fairing())
}