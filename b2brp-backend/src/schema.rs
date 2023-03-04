// @generated automatically by Diesel CLI.

diesel::table! {
    User (id) {
        id -> Int4,
        email -> Text,
        name -> Text,
        password -> Text,
        created_at -> Timestamp,
        salt -> Text,
    }
}
