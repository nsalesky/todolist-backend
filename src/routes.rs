use rocket::response::Debug;
use rocket::response::status;
use rocket::serde::json::{Json};
use rocket_sync_db_pools::diesel;
use rocket_sync_db_pools::diesel::prelude::*;

use crate::database::PostgresDbConn;
use crate::models::{CreateUser, User, InsertableUser};
use crate::schema::users;
use rocket::response::status::Created;

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[post("/users/create", format = "json", data = "<create_user>")]
pub async fn create_user(db: PostgresDbConn, create_user: Json<CreateUser>) -> Result<Created<Json<&'static str>>> {
    let user = InsertableUser::from_create(create_user.into_inner());

    db.run(move |conn| {
        diesel::insert_into(users::table)
            .values(user)
            .execute(conn)
    }).await?;

    Ok(Created::new("/").body(Json("hello there")))
}

#[get("/users/<id>")]
pub async fn get_user(db: PostgresDbConn, id: i32) -> Option<Json<User>> {
    db.run(move |conn| {
        users::table
            .filter(users::id.eq(id))
            .first(conn)
    }).await.map(Json).ok()
}