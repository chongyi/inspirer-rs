use std::rc::Rc;
use actix_web::{HttpRequest, HttpResponse, AsyncResponder, Responder, HttpMessage};

use state::AppState;
use models::content;

#[derive(Deserialize, Debug, Clone)]
struct CreateContent {

}

pub fn create(req: HttpRequest<AppState>) -> impl Responder {
    let req_ref = Rc::new(req);

    ""
}