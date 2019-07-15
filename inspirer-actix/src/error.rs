//! 对 Result 的公共包装
//!
//! 该模块定义了全局项目公共的 `Result` 包装，用于对错误控制的统一管理。通过实现 `CodedError` 实现错误消息
//! 的 `code` 和 `message` 定义。
//!
//! ## 错误代码
//!
//! 错误代码是一个 `i16` 类型的数值。范围在 -128 ~ 128 区间内的错误代码为保留错误代码，其用于作为全局错误代码。
//! 范围之外的错误代码，为业务错误代码，允许在不同业务之间复用（即不同业务内相同代码含义不一样），但同一业务内的
//! 错误代码不能重复。
//!
//! 错误代码的具体含义根据不同业务，可以实行不同规则约束，只需在大体框架（即保留值以外）的限定下即可。
//!
//! ## 错误消息
//!
//! 错误消息是对错误的具体描述，建议一般不要超过 160 个全角字符或 255 个半角字符即可。

use std::error::Error as StdError;
use std::any::TypeId;
use std::fmt::{Formatter, Debug, Display};
pub use http::StatusCode;

/// 未知系统错误代码（或未定义的错误）
pub const UNKNOWN_ERROR_CODE: i16 = -1;
/// 未受控制的系统（或组件）错误代码
pub const UNHANDLE_SYSTEM_ERROR_CODE: i16 = -2;

#[derive(Debug)]
pub struct Error(pub Box<dyn CodedError>);

/// 编入错误代码的错误信息
pub trait CodedError: Debug + Display + Send {
    fn http_status(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    /// 获取错误代码
    fn error_code(&self) -> i16 {
        UNKNOWN_ERROR_CODE
    }

    fn error_message(&self) -> &str {
        "未知异常"
    }
}

impl CodedError for Error {
    fn error_code(&self) -> i16 {
        self.0.as_ref().error_code()
    }

    fn error_message(&self) -> &str {
        self.0.as_ref().error_message()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl CodedError for &'static str {
    fn error_message(&self) -> &str {
        self
    }
}

