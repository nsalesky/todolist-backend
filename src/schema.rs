table! {
    items (item_id) {
        item_id -> Int4,
        list_id -> Int4,
        description -> Text,
        finished -> Bool,
    }
}

table! {
    lists (list_id) {
        list_id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
        date_created -> Date,
    }
}

table! {
    user_lists (id) {
        id -> Int4,
        user_id -> Int4,
        list_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        preferred_name -> Varchar,
        password_hash -> Text,
    }
}

joinable!(items -> lists (list_id));
joinable!(user_lists -> lists (list_id));
joinable!(user_lists -> users (user_id));

allow_tables_to_appear_in_same_query!(
    items,
    lists,
    user_lists,
    users,
);
