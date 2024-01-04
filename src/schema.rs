// @generated automatically by Diesel CLI.
use diesel::{allow_tables_to_appear_in_same_query, table};

table! {
    users(id) {
        id -> Int8,
        client_type -> Text,
        name -> Text,
        avatar -> Text,
        phone -> Text,
    }
}

table! {
    post(id) {
        id -> Int8,
        client_type -> Text,
        name -> Text,
        avatar -> Text,
        phone -> Text,
    }
}

allow_tables_to_appear_in_same_query!(post, users);
