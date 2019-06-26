//! 错误信息定义规范

use crate::prelude::*;
use inspirer_common::result::*;
use inspirer_common::coded_error;
use std::error::Error as StdError;
use std::fmt;

pub type ActionResult<T> = std::result::Result<T, ErrorKind>;

#[derive(Clone, Debug, PartialEq)]
pub struct PaginateWrapper<T> {
    pub data: Vec<T>,
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

impl StdError for ErrorKind {
    fn description(&self) -> &str {
        match self {
            ErrorKind::DBError(err) => err.description(),
            ErrorKind::BizError(err) => err.as_ref().error_message()
        }
    }

    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            ErrorKind::DBError(err) => err.source(),
            ErrorKind::BizError(err) => err.as_ref().source()
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

coded_error!(DeserializeResourceError (10014) "内部资源解析错误");
coded_error!(ForbiddenError (10005) "资源访问被拒绝");
coded_error!(ValidateCodeExistsError (10021) "验证码已创建，指定时间内不可重复创建");