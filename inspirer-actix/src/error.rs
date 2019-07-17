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
use serde::Serialize;
use actix_web::{ResponseError, HttpResponse, HttpRequest};
use actix_web::dev::{HttpResponseBuilder, Body};
use actix_web::http::header;
use crate::response::ResponseMessage;
use derive_more::Display;
pub use actix_web::error::*;

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

impl AsRef<CodedError> for Error {
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
    fn error_response(&self) -> HttpResponse {
        let err = match self {
            InspirerResponseError::Json(err) => err,
            InspirerResponseError::Template(err, _) => err,
        };

        HttpResponse::new(err.http_status())
    }

    fn render_response(&self) -> HttpResponse {
        let mut resp = self.error_response();
        match self {
            InspirerResponseError::Json(err) => {
                let json = serde_json::to_string(&ResponseMessage::error(err, &Option::<String>::None)).unwrap_or_else(|err| {
                    error!("Convert 'error' to json failed, error info: {:?}", err);
                    String::from("{}")
                });
                resp.headers_mut().insert(header::CONTENT_TYPE, header::HeaderValue::from_static("application/json"));
                resp.set_body(Body::from(json))
            },
            InspirerResponseError::Template(err, template) => {
                resp.headers_mut().insert(header::CONTENT_TYPE, header::HeaderValue::from_static("text/html"));
                resp.set_body(Body::from(err.error_message().to_string()))
            }
        }
    }
}

pub fn map_to_inspirer_response_err<T: CodedError + 'static>(req: &HttpRequest) -> impl FnOnce(T) -> actix_web::Error {
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

macro_rules! map_actix_error {
    ($error:path, $code:expr) => {
        impl CodedError for ActixErrorWrapper<$error>
        {
            fn http_status(&self) -> StatusCode {
                self.err.error_response().status()
            }

            fn error_message(&self) -> &str {
                self.msg.as_str()
            }

            fn error_code(&self) -> i16 {
                $code
            }
        }
    };

    ($error:path { $($matcher:pat $(if $pred:expr)* => $result:expr),* }) => {
        impl CodedError for ActixErrorWrapper<$error>
        {
            fn http_status(&self) -> StatusCode {
                self.err.error_response().status()
            }

            fn error_message(&self) -> &str {
                self.msg.as_str()
            }

            fn error_code(&self) -> i16 {
                match &self.err {
                    $($matcher $(if $pred)* => $result),*
                }
            }
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
            BlockingError::Canceled => UNHANDLE_SYSTEM_ERROR_CODE
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