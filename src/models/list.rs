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
}