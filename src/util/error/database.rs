use diesel::result::Error as DieselError;
use diesel::result::DatabaseErrorKind;
use diesel::r2d2::Error as DieselR2d2Error;
use r2d2::Error as R2d2Error;
use actix_web::HttpResponse;

use super::RuntimeCause;
use super::RenderType;
use super::ErrorDesc;
use util::message::ErrorMessage;

const DB_UNKNOWN_ERR: ErrorDesc = (10100, "Unknown database error.");
const DB_CONNECTION_ERR: ErrorDesc = (10101, "Database connection error.");
const DB_DATA_CONFLICT_ERR: ErrorDesc = (10134, "Data conflict.");
const DB_QUERY_ERR: ErrorDesc = (10140, "Database query error.");
const DB_NO_QUERY_RESULT_ERR: ErrorDesc = (10141, "No result.");

impl RuntimeCause for DieselError {
    fn render(&self, render: RenderType) -> HttpResponse {
        let (mut builder, (code, message)) = match *self {
            DieselError::NotFound => (HttpResponse::NotFound(), DB_NO_QUERY_RESULT_ERR),
            DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => (HttpResponse::Conflict(), DB_DATA_CONFLICT_ERR),
            _ => (HttpResponse::InternalServerError(), DB_UNKNOWN_ERR),
        };

        match render {
            RenderType::Json => builder.json(ErrorMessage::<String> {
                code,
                msg: "Database error".to_string(),
                body: Some(message.to_string()),
            }),
            RenderType::Text => builder.body(message),
        }
    }
}

//impl RuntimeCause for DieselR2d2Error {
//    fn render(&self, render: RenderType) -> HttpResponse {
//        let (code, message) = match *self {
//            R2d2Error::ConnectionError(_) => DB_CONNECTION_ERR,
//            R2d2Error::QueryError(_) => DB_QUERY_ERR,
//        };
//
//        let builder = HttpResponse::InternalServerError();
//
//        match render {
//            RenderType::Json => builder.json(ErrorMessage::<String> {
//                code,
//                msg: "Database error".to_string(),
//                body: Some(message.to_string()),
//            }),
//            RenderType::Text => builder.body(message),
//        }
//    }
//}

impl RuntimeCause for R2d2Error {
    fn render(&self, render: RenderType) -> HttpResponse {
        let (code, message) = DB_UNKNOWN_ERR;

        let builder = HttpResponse::InternalServerError();

        match render {
            RenderType::Json => builder.json(ErrorMessage::<String> {
                code,
                msg: "Database error".to_string(),
                body: Some(message.to_string()),
            }),
            RenderType::Text => builder.body(message),
        }
    }
}