use actix_web::*;
use actix_web::middleware::session::RequestSession;
use futures::Future;

use state::AppState;
use util::auth::PrivateClaims;
use util::error::error_handler;
use models::user::FindUser;

pub fn get_current_user_info(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let claims = req.session().get::<PrivateClaims>("claims").unwrap().unwrap();

    req.state().database
        .send(FindUser::Id(claims.uid))
        .from_err()
        .and_then(|res| {
            match res {
                Ok(user) => Ok(HttpResponse::Ok().json(user)),
                Err(e) => Err(e),
            }
        })
        .map_err(error_handler(req))
        .responder()
}