use std::rc::Rc;

use actix_web::*;
use actix_web::middleware::session::RequestSession;
use futures::Future;
use biscuit::*;
use biscuit::jws::*;
use biscuit::jwa::*;
use chrono::{Utc, Duration};
use dotenv;

use state::AppState;
use util::auth::Authentication as Auth;
use util::auth::Email;
use util::auth::{Authenticate, PrivateClaims};

#[derive(Deserialize)]
pub struct Authentication {
    email: String,
    password: String,
}

pub fn authorization(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let cloned_req= req.clone();
    Form::<Authentication>::extract(&req).from_err().and_then(move |r| {
        let auth = Email {
            email: r.email.clone(),
            password: r.password.clone(),
        };

        let checker = Auth::new(auth);

        req.state().database.send(checker).from_err()
    }).and_then(move |res| {
        match res {
            Ok(user) => {
                let chaims = PrivateClaims {
                    uid: user.id,
                    name: user.name,
                };

                Ok(HttpResponse::Ok().body(chaims.generate_jwt_token()?))
            },
            Err(e) => Err(e),
        }

    }).responder()

}