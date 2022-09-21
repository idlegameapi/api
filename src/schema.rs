// @generated automatically by Diesel CLI.

diesel::table! {
    users (token) {
        token -> Text,
        salt -> Varchar,
        balance -> Numeric,
    }
}
