table! {
    cards (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[sql_name = "type"]
        type_ -> Varchar,
        display_name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user_credentials (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[sql_name = "type"]
        type_ -> Varchar,
        login_id -> Bytea,
        login_password -> Bytea,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(cards -> users (user_id));
joinable!(user_credentials -> users (user_id));

allow_tables_to_appear_in_same_query!(cards, user_credentials, users,);
