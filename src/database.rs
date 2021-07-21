use rocket::serde::json::Json;
use rocket_sync_db_pools::{database, diesel};

#[database("postgres_db")]
pub struct PostgresDbConn(diesel::PgConnection);