use chrono::{NaiveDate, Utc};
use diesel::prelude::*;

use crate::schema::lists;
use crate::schema::lists::dsl::*;

/// An object representing a full row in the lists table.
#[derive(Identifiable, Queryable, Serialize, Deserialize)]
#[primary_key(list_id)]
pub struct List {
    pub list_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub date_created: NaiveDate,
}

/// An object representing all the information needed to insert a new row into the lists table.
#[derive(Insertable)]
#[table_name = "lists"]
pub struct NewList {
    pub name: String,
    pub description: Option<String>,
    pub date_created: NaiveDate,
}

/// An object with the information the user needs to provide to create a new list.
#[derive(Serialize, Deserialize)]
pub struct ListDTO {
    pub name: String,
    pub description: Option<String>,
}

impl List {
    /// Attempts to create a new list with the specified information and the current local
    /// date for the "date_created" column.
    /// If the row is created, returns its contents, otherwise returns none.
    pub fn create_list(list: ListDTO, conn: &PgConnection) -> Option<List> {
        let list = NewList {
            name: list.name,
            description: list.description,
            date_created: Utc::now().naive_local().date(),
        };

        let result = diesel::insert_into(lists)
            .values(&list)
            .get_result(conn);

        match result {
            Ok(row) => Some(row),
            Err(_) => None,
        }
    }

    /// Attempts to delete the `List` with the given primary key, if it exists.
    /// Returns true if the delete succeeded, or false if it failed.
    pub fn delete_list(id: i32, conn: &PgConnection) -> bool {
        diesel::delete(lists.filter(list_id.eq(id)))
            .execute(conn)
            .is_ok()
    }

    /// Finds the `List` with the given id, if it exists.
    pub fn find_list_by_id(id: i32, conn: &PgConnection) -> Option<List> {
        let possible_list = lists.filter(list_id.eq(id)).get_result::<List>(conn);

        if let Ok(list) = possible_list {
            Some(list)
        } else {
            None
        }
    }
}