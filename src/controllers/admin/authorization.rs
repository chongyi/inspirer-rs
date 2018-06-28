use actix_web::*;
use futures::Future;

use state::AppState;
use util::auth::Authentication as Auth;
use util::auth::Email;
use util::auth::Authenticate;

#[derive(Deserialize)]
pub struct Authentication {
    email: String,
    password: String,
}

pub fn authorization(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    Form::<Authentication>::extract(&req).from_err().and_then(move |r| {
        let auth = Email {
            email: r.email.clone(),
            password: r.password.clone(),
        };

        let checker = Auth::new(auth);

        req.state().database.send(checker).from_err()
    }).and_then(|res| {
        if res.unwrap() {
            Ok(HttpResponse::Ok().body("ok"))
        } else {
            Ok(HttpResponse::Ok().body("nothing"))
        }
    }).responder()

}