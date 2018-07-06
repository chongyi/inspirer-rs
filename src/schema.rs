table! {
    categories (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        display_name -> Varchar,
        description -> Varchar,
        sort -> Smallint,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    contents (id) {
        id -> Unsigned<Integer>,
        creator_id -> Unsigned<Integer>,
        title -> Varchar,
        category_id -> Nullable<Unsigned<Integer>>,
        keywords -> Varchar,
        description -> Varchar,
        sort -> Unsigned<Smallint>,
        display -> Bool,
        content_type -> Unsigned<Smallint>,
        content_id -> Unsigned<Integer>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    content_articles (id) {
        id -> Unsigned<Integer>,
        content_id -> Nullable<Unsigned<Integer>>,
        content -> Mediumtext,
        name -> Nullable<Varchar>,
        views -> Unsigned<Integer>,
        modified_at -> Nullable<Timestamp>,
    }
}

table! {
    users (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        email -> Varchar,
        password -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    categories,
    contents,
    content_articles,
    users,
);
