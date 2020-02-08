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

use std::any::TypeId;
use std::error::Error as StdError;
use std::fmt::{Debug, Display, Formatter};

pub use actix::MailboxError;
pub use actix_redis::Error as ActixRedisError;
use actix_http::{ResponseBuilder, HttpMessage};
use actix_web::{HttpRequest, HttpResponse, ResponseError};
use actix_web::dev::{Body, HttpResponseBuilder};
pub use actix_web::error::*;
use actix_web::http::header;
use derive_more::Display;
pub use http::StatusCode;
use serde::Serialize;

pub use error_code::*;

use crate::response::ResponseMessage;

#[derive(Debug)]
pub struct Error(pub Box<dyn CodedError>);

/// 编入错误代码的错误信息
pub trait CodedError: Debug + Display + Send {
    fn http_status(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    /// 获取错误代码
    fn error_code(&self) -> i16 {
        UNKNOWN_ERROR_CODE.0
    }

    fn error_message(&self) -> &str {
        UNKNOWN_ERROR_CODE.1
    }
}

impl CodedError for Error {
    fn http_status(&self) -> StatusCode {
        self.0.as_ref().http_status()
    }

    fn error_code(&self) -> i16 {
        self.0.as_ref().error_code()
    }

    fn error_message(&self) -> &str {
        self.0.as_ref().error_message()
    }
}

impl AsRef<dyn CodedError> for Error {
    fn as_ref(&self) -> &(dyn CodedError + 'static) {
        self.0.as_ref()
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

#[derive(Debug, Display)]
pub enum InspirerResponseError {
    #[display(fmt = "{}", _0)]
    Json(Error),
    #[display(fmt = "{}", _0)]
    Template(Error, &'static str)
}

impl ResponseError for InspirerResponseError {
    fn status_code(&self) -> StatusCode {
        let err = match self {
            InspirerResponseError::Json(err) => err,
            InspirerResponseError::Template(err, _) => err,
        };

        err.http_status()
    }

    fn error_response(&self) -> HttpResponse {
        let mut resp = ResponseBuilder::new(self.status_code());
        match self {
            InspirerResponseError::Json(err) => {
                let json = serde_json::to_string(&ResponseMessage::error(err, &Option::<String>::None)).unwrap_or_else(|err| {
                    error!("Convert 'error' to json failed, error info: {:?}", err);
                    String::from("{}")
                });

                resp.set_header(header::CONTENT_TYPE, header::HeaderValue::from_static("application/json"))
                    .body(Body::from(json))
            },
            InspirerResponseError::Template(err, template) => {
                resp.set_header(header::CONTENT_TYPE, header::HeaderValue::from_static("text/html"))
                    .body(Body::from(err.error_message().to_string()))
            }
        }
    }
}

pub fn map_to_inspirer_response_err<T: CodedError + 'static, R: HttpMessage>(req: &R) -> impl FnOnce(T) -> actix_web::Error {
    let json = match req.headers().get(header::ACCEPT) {
        Some(value) => {
            let v = value.to_str().unwrap_or("");
            if v.contains("application/json") {
                true
            } else {
                false
            }
        }
        _ => false
    };

    move |err| {
        let error = Error(Box::new(err));
        if json {
            actix_web::Error::from(InspirerResponseError::Json(error))
        } else {
            actix_web::Error::from(InspirerResponseError::Template(error, ""))
        }
    }
}

#[derive(Debug, Display)]
#[display(fmt = "{}", msg)]
pub struct ActixErrorWrapper<T> {
    err: T,
    msg: String,
}

impl<T: ResponseError + 'static> From<T> for ActixErrorWrapper<T>
{
    fn from(err: T) -> Self {
        let msg = err.to_string();
        ActixErrorWrapper {
            err,
            msg,
        }
    }
}

impl<T: Display + Debug + CodedError> CodedError for BlockingError<T> {
    fn http_status(&self) -> StatusCode {
        match self {
            BlockingError::Error(err) => err.http_status(),
            BlockingError::Canceled => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_code(&self) -> i16 {
        match self {
            BlockingError::Error(err) => err.error_code(),
            BlockingError::Canceled => UNHANDLE_SYSTEM_ERROR_CODE.0
        }
    }

    fn error_message(&self) -> &str {
        match self {
            BlockingError::Error(err) => err.error_message(),
            BlockingError::Canceled => "System blocked",
        }
    }
}

impl CodedError for ParseError {}
impl CodedError for MailboxError {}

pub mod error_code {
    /// 未知系统错误代码（或未定义的错误）
    pub const UNKNOWN_ERROR_CODE: (i16, &'static str) = (-1, "未知异常");
    /// 未受控制的系统（或组件）错误代码
    pub const UNHANDLE_SYSTEM_ERROR_CODE: (i16, &'static str) = (-2, "未知异常（组件错误）");

    /// 非法请求
    pub const INVALID_OR_BAD_REQUEST: (i16, &'static str) = (-3, "非法请求");
    /// 授权校验未通过
    pub const UNAUTHORIZED: (i16, &'static str) = (-4, " 授权校验未通过，请重新获取授权");
}