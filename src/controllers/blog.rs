use actix_web::{HttpRequest, HttpResponse, Responder};
use state::AppState;

pub fn home(req: HttpRequest<AppState>) -> impl Responder {
    "hello world."
}

pub fn content_list() {
    unimplemented!()
}

pub fn content() {
    unimplemented!()
}

pub fn page() {
    unimplemented!()
}

pub fn push_message_list() {
    unimplemented!()
}

pub fn push_message() {
    unimplemented!()
}

pub fn subject_list() {
    unimplemented!()
}

pub fn subject() {
    unimplemented!()
}