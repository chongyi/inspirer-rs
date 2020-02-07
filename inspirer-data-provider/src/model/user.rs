use crate::schema::users;
use chrono::prelude::*;

#[derive(Deserialize, Insertable)]
#[table_name = "users"]
pub struct InsertUser<'i> {
    pub uuid: &'i str,
    pub invitor_uuid: Option<&'i str>,
    pub password: Option<String>,
    pub status: i16,
}

#[derive(AsChangeset)]
#[table_name = "users"]
pub struct UpdateUserLastLogin<'i> {
    pub last_login_ip: Option<&'i str>,
    pub last_login: NaiveDateTime,
}

#[derive(AsChangeset)]
#[table_name = "users"]
pub struct UpdateUserActivatedTime {
    pub activated_at: NaiveDateTime,
}

#[allow(non_upper_case_globals)]
pub const user_base_columns: (
    users::id,
    users::uuid,
    users::status,
    users::user_type
) = (
    users::id,
    users::uuid,
    users::status,
    users::user_type
);

#[allow(non_upper_case_globals)]
pub const user_credential_base: (
    users::uuid,
    users::status,
    users::user_type,
    users::password
) = (
    users::uuid,
    users::status,
    users::user_type,
    users::password
);

#[derive(Queryable, Deserialize, Debug, Clone)]
pub struct UserBase {
    pub id: i64,
    pub uuid: String,
    pub status: i16,
    pub member_type: i16,
}

#[derive(Queryable, Deserialize, Debug, Clone, PartialEq, Serialize)]
pub struct BejoinedUserCredentialBase {
    pub uuid: String,
    pub status: i16,
    pub member_type: i16,
    pub password: Option<String>,
}

#[derive(Queryable, Deserialize, Debug, Clone, PartialEq, Serialize)]
pub struct BeJoinedUserBase {
    pub uuid: Option<String>,
    pub status: Option<i16>,
    pub member_type: Option<i16>,
}