table! {
    messages (id) {
        id -> Int4,
        text -> Text,
        table_users_id -> Int4,
    }
}

table! {
    tokens (id) {
        id -> Int4,
        token -> Text,
        created -> Timestamp,
        table_users_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    messages,
    tokens,
    users,
);
