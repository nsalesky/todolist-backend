#[derive(Debug)]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub email: String,
    pub name: String,
    pub password_hash: String,
}
