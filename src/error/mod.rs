use std::borrow::BorrowMut;
use std::collections::HashMap;
use actix::MailboxError;
use actix_web::{HttpRequest, HttpResponse, HttpMessage, http::StatusCode};
use actix_web::error::{Error as ActixError, ResponseError};
use std::fmt::{self, Formatter};
use message::ErrorMessage;
use tera::{Context, Tera};
use template::TEMPLATES;
use mime;

pub mod database;

pub const UNKNOWN_ERROR: u16 = 65535;
pub const ACTOR_MAILBOX_ERROR: u16 = 60001;

pub fn error_msg<T: Into<String>>(code: u16, msg: T, body: Option<ErrorDetail>) -> ErrorMessage<ErrorDetail> {
    ErrorMessage::<ErrorDetail> {
        code,
        msg: msg.into(),
        body
    }
}

pub fn error_handler<S, T: AsRef<HttpRequest<S>>>(req: T) -> impl FnOnce(Error) -> ActixError {
    let json = if let Ok(Some(mime)) = req.as_ref().mime_type() {
        mime.subtype() == mime::JSON || mime.suffix() == Some(mime::JSON)
    } else {
        false
    };

    move |err: Error| {
        if json {
            let json = JsonError(err);
            json.into()
        } else {
            let html = HtmlError(err);
            html.into()
        }
    }
}

#[derive(Fail, Debug, Serialize)]
#[fail(display = "code: {}, error: {}", code, message)]
pub struct Error {
    #[serde(skip)]
    cause: Box<fmt::Debug + Send + Sync + 'static>,
    code: u16,
    #[serde(skip)]
    status: StatusCode,
    message: String,
    detail: Option<ErrorDetail>
}

impl Default for Error {
    fn default() -> Self {
        Error {
            cause: Box::new("[unknown]"),
            code: 65535,
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Unknown error".into(),
            detail: None
        }
    }
}

#[derive(Serialize, Debug, Deserialize, Clone, Fail)]
#[serde(untagged)]
pub enum ErrorDetail {
    #[fail(display = "[array]")]
    Array(Vec<ErrorDetail>),
    #[fail(display = "[hash map]")]
    Hash(HashMap<String, Option<ErrorDetail>>),
    #[fail(display = "{}", _0)]
    String(String),
}

#[derive(Fail, Debug)]
pub struct JsonError(Error);

#[derive(Fail, Debug)]
pub struct HtmlError(Error);

impl ResponseError for JsonError {

}

impl ResponseError for HtmlError {
    fn error_response(&self) -> HttpResponse {
        let mut context = Context::new();
        context.add("error", &self.0);
        context.add("status", &self.0.status.as_u16());
        let rendered = match TEMPLATES.render("error.html", &context) {
            Ok(r) => r,
            Err(e) => "Render error".into()
        };

        HttpResponse::build(self.0.status).body(rendered)
    }
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for HtmlError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<MailboxError> for Error {
    fn from(err: MailboxError) -> Self {
        Error::internal_server_error(
            Some(err),
            Some(error_msg(ACTOR_MAILBOX_ERROR, "System logic error", None))
        )
    }
}

impl Error {
    pub fn build<T>(err: Option<T>) -> Self
        where T: fmt::Debug + Send + Sync + 'static
    {
        Error {
            cause: Box::new(err),
            ..Default::default()
        }
    }

    pub fn new<T>(err: Option<T>, status: StatusCode, msg: Option<ErrorMessage<ErrorDetail>>) -> Error
        where T: fmt::Debug + Send + Sync + 'static
    {
        match msg {
            Some(m) => {
                Error {
                    cause: Box::new(err),
                    status,
                    code: m.code,
                    message: m.msg,
                    detail: m.body
                }
            },
            None => {
                Self::build(err)
            }
        }
    }

    pub fn internal_server_error<T>(err: Option<T>, msg: Option<ErrorMessage<ErrorDetail>>) -> Error
        where T: fmt::Debug + Send + Sync + 'static
    {
        Self::new(err, StatusCode::INTERNAL_SERVER_ERROR, msg)
    }

    pub fn not_found_error<T>(err: Option<T>, msg: Option<ErrorMessage<ErrorDetail>>) -> Error
        where T: fmt::Debug + Send + Sync + 'static
    {
        Self::new(err, StatusCode::NOT_FOUND, msg)
    }

    pub fn forbidden_error<T>(err: Option<T>, msg: Option<ErrorMessage<ErrorDetail>>) -> Error
        where T: fmt::Debug + Send + Sync + 'static
    {
        Self::new(err, StatusCode::FORBIDDEN, msg)
    }

    pub fn bad_request_error<T>(err: Option<T>, msg: Option<ErrorMessage<ErrorDetail>>) -> Error
        where T: fmt::Debug + Send + Sync + 'static
    {
        Self::new(err, StatusCode::BAD_REQUEST, msg)
    }

    pub fn conflict<T>(err: Option<T>, msg: Option<ErrorMessage<ErrorDetail>>) -> Error
        where T: fmt::Debug + Send + Sync + 'static
    {
        Self::new(err, StatusCode::CONFLICT, msg)
    }
}