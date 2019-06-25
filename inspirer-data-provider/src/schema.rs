table! {
    contents (id) {
        id -> Int8,
        creator_uuid -> Bpchar,
        title -> Nullable<Varchar>,
        content_name -> Nullable<Varchar>,
        content_type -> Int2,
        keywords -> Varchar,
        description -> Varchar,
        display -> Bool,
        published -> Bool,
        published_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int8,
        user_uuid -> Bpchar,
        invitor_uuid -> Nullable<Bpchar>,
        email -> Nullable<Varchar>,
        mobile_phone -> Nullable<Varchar>,
        country_code -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
        nickname -> Varchar,
        gender -> Nullable<Bool>,
        avatar -> Nullable<Varchar>,
        user_type -> Int2,
        last_login -> Nullable<Timestamp>,
        last_login_ip -> Nullable<Varchar>,
        login_count -> Int4,
        status -> Int2,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    validate_codes (id) {
        id -> Int8,
        code -> Varchar,
        validate_target -> Varchar,
        validate_channel -> Int2,
        is_validated -> Bool,
        status -> Bool,
        expired_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    contents,
    users,
    validate_codes,
);
