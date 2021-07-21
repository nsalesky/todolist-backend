table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        preferred_name -> Varchar,
        password_hash -> Text,
        login_session -> Varchar,
    }
}
