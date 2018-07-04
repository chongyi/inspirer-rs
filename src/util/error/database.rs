//use diesel::result::Error as DieselError;
use diesel::result::DatabaseErrorKind as DieselDatabaseErrorKind;
//use actix_web::HttpResponse;
//
//use super::RuntimeCause;
//use super::RenderType;
//use util::message::ErrorMessage;
//
//impl RuntimeCause for DieselError {
//    fn render(&self, render: RenderType) -> HttpResponse {
//        let (mut builder, code, message) = match *self {
//            DieselError::NotFound => (HttpResponse::NotFound(), 10040, "No result"),
//            DieselError::DatabaseError(DieselDatabaseErrorKind::UniqueViolation, _) => (HttpResponse::Conflict(), 10011, "Data conflict."),
//            _ => (HttpResponse::InternalServerError(), 10010, "Unknown database error."),
//        };
//
//        match render {
//            RenderType::Json => self.render_json(builder, ErrorMessage::<String> {
//                code,
//                msg: "Database error".to_string(),
//                body: Some(message.to_string()),
//            }),
//            RenderType::Text => self.render_text(builder, message),
//        }
//    }
//}

pub enum DatabaseErrorKind {
    NotFound(String),
    DatabaseError(DieselDatabaseErrorKind),
}