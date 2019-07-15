//! 错误信息定义规范

use crate::prelude::*;
use inspirer_actix::error::*;
use inspirer_actix::coded_error;
use std::error::Error as StdError;
use std::fmt;
pub use diesel::result::Error as DieselError;

pub type ActionResult<T> = std::result::Result<T, ErrorKind>;

#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct PaginateWrapper<T> {
    pub list: Vec<T>,
    pub last_page: i64,
    pub total: i64,
}

#[derive(Debug)]
pub enum ErrorKind {
    DBError(diesel::result::Error),
    BizError(Box<dyn CodedError>)
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl CodedError for ErrorKind {
    fn error_code(&self) -> i16 {
        match self {
            ErrorKind::DBError(err) => UNHANDLE_SYSTEM_ERROR_CODE,
            ErrorKind::BizError(err) => err.as_ref().error_code()
        }
    }

    fn error_message(&self) -> &str {
        match self {
            ErrorKind::DBError(err) => err.description(),
            ErrorKind::BizError(err) => err.as_ref().error_message()
        }
    }
}

impl From<diesel::result::Error> for ErrorKind {
    fn from(err: diesel::result::Error) -> ErrorKind {
        ErrorKind::DBError(err)
    }
}

impl Into<Error> for ErrorKind {
    fn into(self) -> Error {
        Error(Box::new(self))
    }
}

coded_error!(DeserializeResourceError (10014) http(500) "内部资源解析错误");
coded_error!(ForbiddenError (10005) http(403) "资源访问被拒绝");
coded_error!(ValidateCodeExistsError (10021) http(403) "验证码已创建，指定时间内不可重复创建");