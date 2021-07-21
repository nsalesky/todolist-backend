use rocket::response::Debug;

use todo_backend::hash_password;

use crate::schema::users;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub name: String,
    pub password_hash: String,
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub name: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct InsertableUser {
    pub username: String,
    pub email: String,
    pub name: String,
    pub password_hash: String,
}

impl InsertableUser {
    pub fn from_create(create_user: CreateUser) -> InsertableUser {
        InsertableUser {
            username: create_user.username,
            email: create_user.email,
            name: create_user.name,
            password_hash: hash_password(&create_user.password),
        }
    }
}
