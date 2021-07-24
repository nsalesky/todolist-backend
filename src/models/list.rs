use chrono::{NaiveDate, Utc};
use diesel::pg::Pg;
use diesel::prelude::*;

use crate::models::item::Item;
use crate::schema::*;
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

/// An object containing information for the complete list, including all of its items.
#[derive(Serialize, Deserialize)]
pub struct ListWithItems {
    pub list_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub date_created: NaiveDate,
    pub items: Vec<Item>,
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

        match possible_list {
            Ok(list) => Some(list),
            Err(_) => None,
        }
    }

    /// Finds the complete `ListWithItems` for the list with the given `id`, if it exists.
    pub fn find_complete_list_by_id(id: i32, conn: &PgConnection) -> Option<ListWithItems> {
        if let Some(list) = List::find_list_by_id(id, conn) {
            let items = Item::find_items_for_list(id, conn);

            Some(ListWithItems {
                list_id: list.list_id,
                name: list.name,
                description: list.description,
                date_created: list.date_created,
                items,
            })
        } else {
            None
        }
    }

    /// Finds the `List` rows that the given user can access.
    pub fn find_lists_for_user(user_id: i32, conn: &PgConnection) -> Option<Vec<List>> {
        let possible_lists = lists::table
            .inner_join(user_lists::table.on(lists::list_id.eq(user_lists::list_id)))
            .into_boxed()
            .select((lists::list_id, lists::name, lists::description, lists::date_created))
            .filter(user_lists::user_id.eq(user_id))
            .load::<List>(conn);

        match possible_lists {
            Ok(result_lists) => Some(result_lists),
            Err(_) => None,
        }
    }
}