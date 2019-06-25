use crate::prelude::*;
use crate::schema::validate_codes;
use chrono::prelude::*;

#[derive(Insertable, Deserialize)]
#[table_name = "validate_codes"]
pub struct ValidateCodeInsert<'i> {
    pub code: &'i str,
    pub validate_target: &'i str,
    pub validate_channel: i16,
    pub expired_at: Option<NaiveDateTime>,
}