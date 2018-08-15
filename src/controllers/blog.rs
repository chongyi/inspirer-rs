use actix_web::{HttpRequest, HttpResponse, Responder};
use state::AppState;

pub fn home(req: HttpRequest<AppState>) -> impl Responder {
    "hello world."
}

pub fn content_list() {

}

pub fn content() {

}

pub fn page() {

}

pub fn push_message_list() {

}

pub fn push_message() {

}

pub fn subject_list() {

}

pub fn subject() {

}