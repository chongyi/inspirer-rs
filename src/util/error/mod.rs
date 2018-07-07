use std::collections::HashMap;
use std::fmt;

use actix_web::{Error, ResponseError, HttpRequest, HttpResponse, HttpMessage, http::header::{HeaderValue, ACCEPT}};
use actix_web::http::{StatusCode};
use actix::MailboxError;
use failure::{Fail, Backtrace};
use dotenv;

pub fn error_handler<T>(r: T) -> impl FnOnce(ApplicationError) -> Error
    where T: Into<RenderType>
{
    let rt: RenderType = r.into();
    move |err: ApplicationError| -> Error {
        let info = match err {
            ApplicationError::SystemRuntimeError(info) => info,
            ApplicationError::AuthenticationError(info) => info,
            ApplicationError::DatabaseError(_, info) => info,
        };

        match rt {
            RenderType::Json => JsonResponseError::new(info).into(),
            RenderType::Text => TextResponseError::new(info).into(),
        }
    }
}

#[macro_export]
macro_rules! error_trigger_define {
    ($ct:ident, $code:expr, $msg:expr, $status:expr) => {
        pub const $ct: (u16, &'static str, StatusCode) = ($code, $msg, $status);
    };
    ($rt:expr, $ct:ident, __, $method:ident) => {
        #[allow(non_snake_case)]
        pub fn $method() -> Self {
            $rt(ErrorInformation::new(
                Self::$ct.0, Self::$ct.1.into(), Self::$ct.2, None
            ))
        }
    };
    ($rt:expr, $ct:ident, $en:expr, $method:ident) => {
        #[allow(non_snake_case)]
        pub fn $method() -> Self {
            $rt($en, ErrorInformation::new(
                Self::$ct.0, Self::$ct.1.into(), Self::$ct.2, None
            ))
        }
    };
    ($rt:expr, $ct:ident, $code:expr, $msg:expr, $status:expr, __, $method:ident) => {
        error_trigger_define!($ct, $code, $msg, $status);
        error_trigger_define!($rt, $ct, __, $method);
    };
    ($rt:expr, $ct:ident, $code:expr, $msg:expr, $status:expr, $en:expr, $method:ident) => {
        error_trigger_define!($ct, $code, $msg, $status);
        error_trigger_define!($rt, $ct, $en, $method);
    };
}

#[macro_use]
pub mod database;
pub mod auth;

use super::message::ErrorMessage;
use self::database::DatabaseErrorKind;

pub enum RenderType {
    Json,
    Text,
}

impl<S> From<HttpRequest<S>> for RenderType {
    fn from(req: HttpRequest<S>) -> Self {
        let default = HeaderValue::from_static("text/html");
        let accept = req.headers()
            .get(ACCEPT).unwrap_or(&default)
            .to_str().unwrap_or("text/html");

        if accept.contains("json") {
            RenderType::Json
        } else {
            RenderType::Text
        }
    }
}

pub enum ApplicationError {
    SystemRuntimeError(ErrorInformation),
    AuthenticationError(ErrorInformation),
    DatabaseError(DatabaseErrorKind, ErrorInformation),
}

impl From<Error> for ApplicationError {
    fn from(err: Error) -> Self {
        ApplicationError::SysRuntimeError(err)
    }
}

impl fmt::Debug for ApplicationError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error")
    }
}

impl fmt::Display for ApplicationError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error")
    }
}

impl From<MailboxError> for ApplicationError {
    fn from(_: MailboxError) -> Self {
        ApplicationError::SysLogicArgumentError()
    }
}

#[derive(Fail, Serialize, Clone)]
pub enum ErrorDetail {
    String(String),
    Array(HashMap<String, ErrorDetail>),
    List(Vec<String>),
}

impl fmt::Display for ErrorDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Debug for ErrorDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let detail: String = match self {
            ErrorDetail::String(s) => s.clone(),
            ErrorDetail::Array(h) => format!("{:?}", h),
            ErrorDetail::List(v) => v.join("\n"),
        };

        write!(f, "{}", detail)
    }
}

#[derive(Fail, Debug)]
pub struct ErrorInformation {
    code: u16,
    status: StatusCode,
    message: String,
    detail: Option<ErrorDetail>,
}

impl ErrorInformation {
    pub fn new(code: u16, message: String, status: StatusCode, detail: Option<ErrorDetail>) -> Self {
        ErrorInformation { code, status, message, detail}
    }
}

impl Default for ErrorInformation {
    fn default() -> Self {
        ErrorInformation {
            code: 65535,
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: String::from("Unknown server error."),
            detail: None,
        }
    }
}

impl fmt::Display for ErrorInformation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let detail: String = if let Some(ref dt) = self.detail {
            format!("{}", dt)
        } else {
            String::from("No more information")
        };

        write!(f, "*Error*({}) {} \n{:?}", self.code, self.message, detail)
    }
}

pub struct JsonResponseError {
    info: ErrorInformation,
    backtrace: Backtrace,
}

impl JsonResponseError {
    pub fn new(info: ErrorInformation) -> Self {
        JsonResponseError {
            info,
            backtrace: Backtrace::new(),
        }
    }
}

impl ResponseError for JsonResponseError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.info.status).json(
            ErrorMessage::new(
                self.info.code,
                self.info.message.clone(),
                {
                    if dotenv::var("DEBUG").unwrap_or("false".to_string()).to_lowercase() == "true" {
                        Some(self.info.detail.clone())
                    } else {
                        None
                    }
                }
            )
        )
    }
}

impl Fail for JsonResponseError
{
    fn backtrace(&self) -> Option<&Backtrace> {
        Some(&self.backtrace)
    }
}

impl fmt::Debug for JsonResponseError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.info, f)
    }
}

impl fmt::Display for JsonResponseError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.info, f)
    }
}

pub struct TextResponseError {
    info: ErrorInformation,
    backtrace: Backtrace,
}

impl TextResponseError {
    pub fn new(info: ErrorInformation) -> Self {
        TextResponseError {
            info,
            backtrace: Backtrace::new(),
        }
    }
}

impl ResponseError for TextResponseError {
    fn error_response(&self) -> HttpResponse {
        let detail: String = if let Some(ref dt) = self.info.detail {
            format!("{}", dt)
        } else {
            String::from("No more information")
        };

        HttpResponse::build(self.info.status).body(
            format!(
                "<p><strong>Error</strong>({}) {}</p>\n<p><pre>{}</pre></p>",
                self.info.code,
                self.info.message,
                detail
            )
        )
    }
}

impl Fail for TextResponseError
{
    fn backtrace(&self) -> Option<&Backtrace> {
        Some(&self.backtrace)
    }
}

impl fmt::Debug for TextResponseError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.info, f)
    }
}

impl fmt::Display for TextResponseError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.info, f)
    }
}

impl ApplicationError {
    error_trigger_define!(SYS_RUNTIME_ERR, 10000, "Unknown runtime error.", StatusCode::INTERNAL_SERVER_ERROR);
    error_trigger_define!(ApplicationError::SystemRuntimeError, SYS_LOGIC_ARG_ERR, 10010, "Invalid argument.", StatusCode::INTERNAL_SERVER_ERROR, __, SysLogicArgumentError);
    error_trigger_define!(ApplicationError::SystemRuntimeError, SYS_INVALID_ARGUMENT_ERR, 10011, "Invalid argument.", StatusCode::BAD_REQUEST, __, SysInvalidArgumentError);

    #[allow(non_snake_case)]
    pub fn SysRuntimeError<T: fmt::Debug>(context: T) -> Self {
        let detail = format!("{:?}", context);
        let string_list = detail.split("\n").map(|x| x.to_string()).collect();

        ApplicationError::SystemRuntimeError(ErrorInformation {
            code: Self::SYS_RUNTIME_ERR.0,
            message: Self::SYS_RUNTIME_ERR.1.into(),
            status: Self::SYS_RUNTIME_ERR.2,
            detail: Some(ErrorDetail::List(string_list)),
        })
    }
}