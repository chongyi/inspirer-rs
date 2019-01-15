table! {
    categories (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        display_name -> Varchar,
        keywords -> Varchar,
        description -> Varchar,
        sort -> Smallint,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    contents (id) {
        id -> Unsigned<Integer>,
        name -> Nullable<Varchar>,
        cover -> Nullable<Varchar>,
        title -> Varchar,
        category_id -> Nullable<Unsigned<Integer>>,
        as_page -> Bool,
        allow_comment -> Bool,
        limit_comment -> Tinyint,
        keywords -> Varchar,
        description -> Varchar,
        sort -> Smallint,
        content_type -> Unsigned<Smallint>,
        content -> Nullable<Mediumtext>,
        display -> Bool,
        published_at -> Nullable<Timestamp>,
        modified_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    links (id) {
        id -> Unsigned<Integer>,
        title -> Varchar,
        link -> Varchar,
        sort -> Unsigned<Smallint>,
        icon -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    members (id) {
        id -> Unsigned<Integer>,
        nickname -> Varchar,
        avatar -> Nullable<Varchar>,
        gender -> Tinyint,
        email -> Varchar,
        email_is_valid -> Bool,
        email_verified_at -> Nullable<Timestamp>,
        password -> Nullable<Varchar>,
        status -> Tinyint,
        activated_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    push_messages (id) {
        id -> Unsigned<Integer>,
        content -> Varchar,
        allow_comment -> Bool,
        limit_comment -> Tinyint,
        sort -> Smallint,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    recommend_contents (id) {
        id -> Unsigned<Integer>,
        content_id -> Nullable<Unsigned<Integer>>,
        source -> Varchar,
        title -> Varchar,
        summary -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    subjects (id) {
        id -> Unsigned<Integer>,
        name -> Nullable<Varchar>,
        cover -> Nullable<Varchar>,
        title -> Varchar,
        keywords -> Varchar,
        description -> Varchar,
        sort -> Smallint,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    subject_relates (subject_id, content_id) {
        subject_id -> Unsigned<Integer>,
        content_id -> Unsigned<Integer>,
        sort -> Smallint,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

allow_tables_to_appear_in_same_query!(
    categories,
    contents,
    links,
    members,
    push_messages,
    recommend_contents,
    subjects,
    subject_relates,
);
