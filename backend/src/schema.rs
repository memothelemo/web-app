// @generated automatically by Diesel CLI.

diesel::table! {
    letters (id) {
        id -> Uuid,
        created_at -> Timestamp,
        author -> Varchar,
        message -> Text,
        secret -> Bool,
    }
}

diesel::table! {
    reports (id) {
        id -> Int8,
        email -> Varchar,
        created_at -> Timestamp,
        letter_id -> Uuid,
        #[sql_name = "type"]
        type_ -> Int4,
        details -> Text,
        resolved -> Bool,
    }
}

diesel::table! {
    states (id) {
        id -> Int4,
        available -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        created_at -> Timestamp,
        name -> Varchar,
        password -> Text,
        moderator -> Nullable<Bool>,
    }
}

diesel::joinable!(reports -> letters (letter_id));

diesel::allow_tables_to_appear_in_same_query!(
    letters,
    reports,
    states,
    users,
);
