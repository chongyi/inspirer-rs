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
        title -> Varchar,
        category_id -> Nullable<Unsigned<Integer>>,
        as_page -> Bool,
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
    push_messages (id) {
        id -> Unsigned<Integer>,
        content -> Varchar,
        sort -> Smallint,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    subjects (id) {
        id -> Unsigned<Integer>,
        name -> Nullable<Varchar>,
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
    push_messages,
    subjects,
    subject_relates,
);
