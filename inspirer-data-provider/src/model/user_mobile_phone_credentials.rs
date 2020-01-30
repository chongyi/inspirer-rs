use crate::schema::user_mobile_phone_credentials;
use chrono::prelude::*;

#[derive(Deserialize, Insertable)]
#[table_name = "user_mobile_phone_credentials"]
pub struct InsertUserMobilePhoneCredential<'i> {
    pub user_uuid: &'i str,
    pub country_code: &'i str,
    pub mobile_phone: &'i str,
    pub status: i16,
}