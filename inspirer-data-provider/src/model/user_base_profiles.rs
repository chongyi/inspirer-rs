use crate::schema::user_base_profiles;
use chrono::prelude::*;

#[derive(Queryable, Deserialize, Debug, Clone, PartialEq, Serialize)]
pub struct BeJoinedUserBaseProfile {
    pub nickname: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Deserialize, Insertable)]
#[table_name = "user_base_profiles"]
pub struct InsertUserBaseProfile<'i> {
    pub user_uuid: &'i str,
    pub nickname: &'i str,
    pub avatar: &'i str,
    pub gender: i16,
}