#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde;

use rocket::{Build, Request};

mod database;
mod models;
mod routes;
mod schema;
mod auth;
mod constants;
mod services;

#[catch(404)]
fn not_found(request: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", request.uri())
}

/// Builds the Rocket instance to use.
pub fn rocket() -> rocket::Rocket<Build> {
    rocket::build()
        .mount("/api",
        routes![
            routes::users::signup,
            routes::users::login,
            routes::users::get_user,

            routes::lists::create_list,
            routes::lists::delete_list,
            routes::lists::post_item,
            routes::lists::delete_item,
        ])
        .attach(database::PostgresDbConn::fairing())
        .register("/api", catchers![not_found])
}