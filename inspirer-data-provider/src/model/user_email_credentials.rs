use crate::schema::user_email_credentials;
use chrono::prelude::*;

#[derive(Deserialize, Insertable)]
#[table_name = "user_email_credentials"]
pub struct InsertUserEmailCredential<'i> {
    pub user_uuid: &'i str,
    pub email: &'i str,
    pub status: i16,
    pub activated_at: Option<NaiveDateTime>,
}