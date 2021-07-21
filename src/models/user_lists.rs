use diesel::prelude::*;
use crate::models::user::{User, UserDTO};
use crate::models::list::{List, ListDTO};
use crate::schema::user_lists::dsl::*;
use crate::schema::user_lists;

/// Represents a single row in the user_lists table associating a user with a list that they
/// have access to.
#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct UserList {
    pub id: i32,
    pub user_id: i32,
    pub list_id: i32,
}

/// Represents a new row that can be inserted into the users_lists table.
#[derive(Insertable)]
#[table_name = "user_lists"]
pub struct NewUserList {
    pub user_id: i32,
    pub list_id: i32,
}

impl UserList {
    /// Associates the given list and user in the user_lists table.
    pub fn associate_list(list: List, user: User, conn: &PgConnection) -> bool {
        let user_list = NewUserList {
            user_id: user.id,
            list_id: list.list_id,
        };

        diesel::insert_into(user_lists)
            .values(&user_list)
            .execute(conn)
            .is_ok()
    }
}