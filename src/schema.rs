table! {
    categories (id) {
        id -> Unsigned<Integer>,
        parent_id -> Unsigned<Integer>,
        path -> Varchar,
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
        creator_id -> Unsigned<Integer>,
        creator_nickname -> Varchar,
        creator_avatar -> Nullable<Varchar>,
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
        mobile_phone -> Nullable<Varchar>,
        country_code -> Varchar,
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
    platform_accounts (id) {
        id -> Unsigned<Integer>,
        account -> Varchar,
        password -> Nullable<Varchar>,
        is_system -> Bool,
        email -> Varchar,
        email_is_valid -> Bool,
        email_verified_at -> Nullable<Timestamp>,
        member_id -> Nullable<Unsigned<Integer>>,
        member_bound_at -> Nullable<Timestamp>,
        role_id -> Unsigned<Integer>,
        status -> Bool,
        activated_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    platform_permissions (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        display_name -> Varchar,
        description -> Varchar,
        model -> Varchar,
        model_parameters -> Nullable<Text>,
        is_system -> Bool,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    platform_permission_relates (target_id, target_type, permission_id) {
        target_id -> Unsigned<Integer>,
        target_type -> Tinyint,
        permission_id -> Unsigned<Integer>,
        status -> Bool,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    platform_roles (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        display_name -> Varchar,
        description -> Varchar,
        icon -> Nullable<Varchar>,
        is_system -> Bool,
        is_super -> Bool,
        status -> Bool,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    platform_role_relates (account_id, role_id) {
        account_id -> Unsigned<Integer>,
        role_id -> Unsigned<Integer>,
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
    platform_accounts,
    platform_permissions,
    platform_permission_relates,
    platform_roles,
    platform_role_relates,
    push_messages,
    recommend_contents,
    subjects,
    subject_relates,
);
