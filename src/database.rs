use rocket::serde::json::Json;
use rocket_sync_db_pools::{database, diesel};

#[database("postgres_db")]
pub struct PostgresDbConn(diesel::PgConnection);

// /// Attempts to get the user information for the user with the specified username if they exist
// pub async fn get_user(username: String, db:PostgresDbConn) -> Option<User> {
//     db.run(move |client: &mut Client| {
//         let query_results = client.query("SELECT * FROM users WHERE username = $1::TEXT", &[&username]);
//
//         let query_results = query_results.unwrap_or(vec![]);
//
//         if query_results.len() == 0 {
//             return None;
//         }
//
//         let row = &query_results[0];
//
//         let user = User {
//             user_id: row.get("user_id"),
//             username: row.get("username"),
//             email: row.get("email"),
//             name: row.get("name"),
//             password_hash: row.get("password_hash"),
//         };
//
//         Some(user)
//     }).await
// }