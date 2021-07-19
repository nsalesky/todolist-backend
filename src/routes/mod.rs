use rocket::response::content::Json;
use crate::database;
use rocket::serde::json::Value;
use crate::user::CreateUser;

#[get("/users/<username>")]
pub async fn get_user(username: String, conn: database::PostgresDbConn) -> Option<Json<String>> {
    let user = database::get_user(username, conn).await;

    match user {
        Some(user) => {
            let str = format!("{user_id}: {name}, {username}, {email}",
                              user_id = user.user_id, name = user.name, username = user.username,
                              email = user.email);

            Some(Json(str))
        },
        None => None
    }
}

#[post("/users/create", format = "json", data = "<create_user>")]
pub async fn create_user(create_user: CreateUser, conn: database::PostgresDbConn) -> Value {

}