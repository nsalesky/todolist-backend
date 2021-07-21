use rocket::serde::json::Json;
use rocket_sync_db_pools::{database, diesel};

/// The wrapper around the database connection that allows it to be pooled.
#[database("postgres_db")]
pub struct PostgresDbConn(diesel::PgConnection);