use actix_web::*;
use actix_web::middleware::session::RequestSession;
use futures::Future;

use state::AppState;
use util::auth::PrivateClaims;
use util::error::runtime_error_container;
use models::user::FindUser;

pub fn get_current_user_info(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let claims = req.session().get::<PrivateClaims>("claims").unwrap().unwrap();

    req.clone().state().database
        .send(FindUser {
            id: Some(claims.uid),
            email: None,
        })
        .from_err()
        .and_then(move |res| {
            let user = res.map_err(runtime_error_container(req).into())?;
            Ok(HttpResponse::Ok().json(user))
        })
        .responder()
}