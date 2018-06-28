table! {
    articles (id) {
        id -> Unsigned<Integer>,
        creator_id -> Unsigned<Integer>,
        title -> Varchar,
        category_id -> Unsigned<Integer>,
        keywords -> Varchar,
        description -> Varchar,
        content -> Mediumtext,
        sort -> Unsigned<Smallint>,
        name -> Nullable<Varchar>,
        views -> Unsigned<Integer>,
        display -> Unsigned<Smallint>,
        modified_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

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
    articles,
    categories,
    users,
);
