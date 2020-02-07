use crate::schema::user_email_credentials;
use chrono::prelude::*;

#[allow(non_upper_case_globals)]
pub const user_email_credential_base: (
    user_email_credentials::user_uuid,
    user_email_credentials::email,
    user_email_credentials::status
) = (
    user_email_credentials::user_uuid,
    user_email_credentials::email,
    user_email_credentials::status
);

#[derive(Deserialize, Insertable)]
#[table_name = "user_email_credentials"]
pub struct InsertUserEmailCredential<'i> {
    pub user_uuid: &'i str,
    pub email: &'i str,
    pub status: i16,
    pub activated_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct UserEmailCredentialBase {
    pub user_uuid: String,
    pub email: String,
    pub status: i16,
}