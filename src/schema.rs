// @generated automatically by Diesel CLI.
use diesel::{allow_tables_to_appear_in_same_query, table};

table! {
    users(id) {
        id -> BigSerial,
        name -> Text,
        avatar -> Text,
        phone -> Text,
        created_at->BigInt,
        updated_at->BigInt,
        deleted_at -> Nullable<BigInt>,
    }
}

table! {
    post(id) {
        id -> Int8,
        name -> Text,
        avatar -> Text,
        phone -> Text,
    }
}

allow_tables_to_appear_in_same_query!(post, users);
