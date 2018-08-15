use actix_web::{HttpRequest, HttpResponse, Responder};
use state::AppState;

pub fn home(req: HttpRequest<AppState>) -> impl Responder {
    "hello world."
}