use diesel::prelude::*;
use diesel::result::Error;

use crate::models::list::{List, ListDTO};
use crate::models::user::{User, UserDTO};
use crate::schema::user_lists;
use crate::schema::user_lists::dsl;

/// Represents a single row in the user_lists table associating a user with a list that they
/// have access to.
#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct UserList {
    pub id: i32,
    pub user_id: i32,
    pub list_id: i32,
    pub is_owner: bool,
}

/// Represents a new row that can be inserted into the users_lists table.
#[derive(Insertable)]
#[table_name = "user_lists"]
pub struct NewUserList {
    pub user_id: i32,
    pub list_id: i32,
    pub is_owner: bool,
}

impl UserList {
    /// Associates the given list and user in the user_lists table.
    pub fn associate_list(list: List, user: User, is_owner: bool, conn: &PgConnection) -> bool {
        let user_list = NewUserList {
            user_id: user.id,
            list_id: list.list_id,
            is_owner,
        };

        diesel::insert_into(dsl::user_lists)
            .values(&user_list)
            .execute(conn)
            .is_ok()
    }

    /// Determines whether or not the user with ID `user_id` owns the list with ID `list_id`.
    pub fn is_list_owner(list_id: i32, user_id: i32, conn: &PgConnection) -> bool {
        let row: QueryResult<UserList> = dsl::user_lists
            .filter(dsl::user_id.eq(&user_id))
            .filter(dsl::list_id.eq(&list_id))
            .get_result::<UserList>(conn);

        match row {
            Ok(user_list) => user_list.is_owner,
            Err(_) => false,
        }
    }

    /// Determines whether or not the user with ID `user_id` has access to edit the list with ID `list_id`.
    /// Note: a user doesn't have to own a list in order to edit it
    pub fn has_list_access(list_id: i32, user_id: i32, conn: &PgConnection) -> bool {
        let row: QueryResult<UserList> = dsl::user_lists
            .filter(dsl::user_id.eq(&user_id))
            .filter(dsl::list_id.eq(&list_id))
            .get_result::<UserList>(conn);

        match row {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    /// Finds the `UserList` row connecting the list with the given `list_id` and the user with the
    /// given `user_id` if it exists.
    // pub fn find_connecting_row(list_id: i32, user_id: i32, conn: &PgConnection) -> Option<UserList> {
    //     let row: QueryResult<UserList> = dsl::user_lists
    //         .filter(dsl::user_id.eq(&user_id))
    //         .filter(dsl::list_id.eq(&list_id))
    //         .get_result::<UserList>(conn);
    //
    //     match row {
    //         Ok(user_list) => Some(user_list),
    //         Err(_) => None,
    //     }
    // }

    /// Attempts to delete the `UserList` with the given id.
    /// Returns true if the delete was successful and false otherwise.
    pub fn delete_user_list(id: i32, conn: &PgConnection) -> bool {
        diesel::delete(dsl::user_lists.filter(dsl::id.eq(id)))
            .execute(conn)
            .is_ok()
    }
}