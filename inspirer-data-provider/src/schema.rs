table! {
    content_entities (content_uuid, version) {
        content_uuid -> Bpchar,
        version -> Varchar,
        content_body -> Nullable<Text>,
        creator_uuid -> Nullable<Bpchar>,
    }
}

table! {
    contents (id) {
        id -> Int8,
        version -> Varchar,
        uuid -> Bpchar,
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
    user_base_profiles (user_uuid) {
        user_uuid -> Bpchar,
        nickname -> Varchar,
        avatar -> Varchar,
        gender -> Int2,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user_email_credentials (email) {
        email -> Varchar,
        user_uuid -> Bpchar,
        status -> Int2,
        activated_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user_mobile_phone_credentials (country_code, mobile_phone) {
        country_code -> Varchar,
        mobile_phone -> Varchar,
        status -> Int2,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int8,
        uuid -> Bpchar,
        invitor_uuid -> Nullable<Bpchar>,
        password -> Nullable<Varchar>,
        user_type -> Int2,
        last_login -> Nullable<Timestamp>,
        last_login_ip -> Nullable<Varchar>,
        login_count -> Int4,
        status -> Int2,
        activated_at -> Nullable<Timestamp>,
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
    content_entities,
    contents,
    user_base_profiles,
    user_email_credentials,
    user_mobile_phone_credentials,
    users,
    validate_codes,
);
