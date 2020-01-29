use uuid::Uuid;
use diesel::r2d2::{self, ConnectionManager};
use diesel::prelude::*;
use crate::db::{EventHandler, ErrorHandler, ConnectionMeta, ErrorHandlerWrapper, EventHandlerWrapper};
use crate::prelude::*;
use chrono::prelude::*;

/// 生成 UUID
pub fn generate_uuid(uuid: &mut [u8]) -> &mut str {
    Uuid::new_v4().to_simple().encode_lower(uuid)
}

pub fn password_hash<P: AsRef<[u8]>>(password: P) -> String {
    pwhash::bcrypt::hash(password).unwrap()
}

pub fn convert_to_native_datetime<T: AsRef<str>>(source: T) -> Result<NaiveDateTime, result::ErrorKind> {
    Ok(Utc.datetime_from_str(source.as_ref(), "%Y-%m-%d %H:%M:%S")
        .map_err(|_| utils::biz_err(result::DeserializeResourceError))?
        .naive_local())
}

pub fn create_pool_builder_with_handler(
    meta: ConnectionMeta,
    error_handler: Box<dyn ErrorHandler<<ConnectionManager<PgConnection> as r2d2::ManageConnection>::Error>>,
    event_handler: Box<dyn EventHandler>,
) -> r2d2::Builder<ConnectionManager<PgConnection>> {
    r2d2::Pool::builder()
        .error_handler(Box::new(ErrorHandlerWrapper {
            meta: meta.clone(),
            handler: error_handler,
        }))
        .event_handler(Box::new(EventHandlerWrapper {
            meta: meta.clone(),
            handler: event_handler,
        }))
}

pub fn biz_err(err: impl CodedError + 'static) -> ErrorKind {
    ErrorKind::BizError(Box::new(err))
}