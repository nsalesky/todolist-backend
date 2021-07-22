use diesel::prelude::*;
use crate::schema::items;
use crate::schema::items::dsl;
use diesel::pg::Pg;

/// An object representing a complete row in the items table.
#[derive(Identifiable, Queryable, Serialize, Deserialize)]
#[primary_key("item_id")]
pub struct Item {
    pub item_id: i32,
    pub list_id: i32,
    pub description: String,
    pub finished: bool,
}

/// A transfer object representing information for the item that can be updated by the user.
#[derive(Serialize, Deserialize)]
pub struct ItemDTO {
    pub description: String,
    pub finished: bool,
}

/// An item that can be inserted into the items table.
#[derive(Insertable)]
#[table_name = "items"]
pub struct NewItem {
    pub list_id: i32,
    pub description: String,
    pub finished: bool,
}

impl Item {
    /// Attempts to insert the given `item` into the items table with reference to the given
    /// `list_id`. Returns true if successful and false otherwise.
    pub fn create_item_for_list(item: ItemDTO, list_id: i32, conn: &PgConnection) -> bool {
        let item = NewItem {
            list_id,
            description: item.description,
            finished: item.finished,
        };

        diesel::insert_into(dsl::items)
            .values(&item)
            .execute(conn)
            .is_ok()
    }

    /// Attempts to delete the item with the given `id`. Returns true if successful or
    /// false otherwise.
    pub fn delete_item(id: i32, conn: &PgConnection) -> bool {
        diesel::delete(dsl::items.filter(dsl::item_id.eq(id)))
            .execute(conn)
            .is_ok()
    }

    /// Determines whether or not the given item is contained in the given list.
    pub fn owned_by_list(item_id: i32, list_id: i32, conn: &PgConnection) -> bool {
        let possible_item = dsl::items.filter(dsl::item_id.eq(item_id)).get_result::<Item>(conn);

        if let Ok(item) = possible_item {
            item.list_id == list_id
        } else {
            false
        }
    }

    // pub fn find_item_by_id(id: i32, conn: &PgConnection) -> Option<Item> {
    //     let possible_item = dsl::items.filter(dsl::item_id.eq(user_id)).get_result::<User>(conn);
    //     if let Ok(user) = possible_user {
    //         Some(user)
    //     } else {
    //         None
    //     }
    // }
}