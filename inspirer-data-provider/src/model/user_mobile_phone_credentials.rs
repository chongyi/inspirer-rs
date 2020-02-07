use crate::schema::user_mobile_phone_credentials;
use chrono::prelude::*;

#[allow(non_upper_case_globals)]
pub const user_mobile_phone_credential_base: (
    user_mobile_phone_credentials::user_uuid,
    user_mobile_phone_credentials::country_code,
    user_mobile_phone_credentials::mobile_phone,
    user_mobile_phone_credentials::status
) = (
    user_mobile_phone_credentials::user_uuid,
    user_mobile_phone_credentials::country_code,
    user_mobile_phone_credentials::mobile_phone,
    user_mobile_phone_credentials::status
);

#[derive(Deserialize, Insertable)]
#[table_name = "user_mobile_phone_credentials"]
pub struct InsertUserMobilePhoneCredential<'i> {
    pub user_uuid: &'i str,
    pub country_code: &'i str,
    pub mobile_phone: &'i str,
    pub status: i16,
}

#[derive(Queryable, Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct UserMobilePhoneCredentialBase {
    pub user_uuid: String,
    pub country_code: String,
    pub mobile_phone: String,
    pub status: i16,
}