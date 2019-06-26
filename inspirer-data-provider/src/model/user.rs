use crate::schema::users;
use chrono::prelude::*;

#[derive(Deserialize, Insertable)]
#[table_name = "users"]
pub struct MobilePhoneRegisterUser<'i> {
    pub user_uuid: &'i str,
    pub invitor_uuid: Option<&'i str>,
    pub mobile_phone: &'i str,
    pub country_code: &'i str,
    pub password: Option<String>,
    pub nickname: &'i str,
    pub status: i16,
}

#[derive(Deserialize, Insertable)]
#[table_name = "users"]
pub struct EmailRegisterUser<'i> {
    pub user_uuid: &'i str,
    pub invitor_uuid: Option<&'i str>,
    pub email: &'i str,
    pub password: Option<String>,
    pub nickname: &'i str,
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
    users::user_uuid,
    users::nickname,
    users::avatar,
    users::status,
    users::user_type
) = (
    users::id,
    users::user_uuid,
    users::nickname,
    users::avatar,
    users::status,
    users::user_type
);

#[derive(Queryable, Deserialize, Debug, Clone)]
pub struct UserBase {
    pub id: i64,
    pub user_uuid: String,
    pub nickname: String,
    pub avatar: Option<String>,
    pub status: i16,
    pub member_type: i16,
}

#[derive(Queryable, Deserialize, Debug, Clone, PartialEq)]
pub struct BeJoinedUserBase {
    pub id: Option<i64>,
    pub user_uuid: Option<String>,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub status: Option<i16>,
    pub member_type: Option<i16>,
}