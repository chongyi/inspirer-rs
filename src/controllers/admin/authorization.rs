use actix_web::*;
use futures::Future;

use state::AppState;
use util::auth::{Authentication as Auth, PrivateClaims, Email};

#[derive(Deserialize)]
pub struct Authentication {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct AuthorizationResult {
    token: String,
    exp: i64,
}

pub fn authorization(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let origin = req.clone();
    Json::<Authentication>::extract(&req).from_err().and_then(move |r| {
        let auth = Email {
            email: r.email.clone(),
            password: r.password.clone(),
        };

        let checker = Auth::new(auth);

        req.state().database.send(checker).from_err()
    }).and_then(|res| {
        match res {
            Ok(user) => {
                let claims = PrivateClaims {
                    uid: user.id,
                    name: user.name,
                };

                let (token, expired) = claims.generate_jwt_token()?;

                Ok(HttpResponse::Ok().json(AuthorizationResult {
                    token,
                    exp: expired,
                }))
            },
            Err(e) => Err(e),
        }

    }).map_err(error_handler!(origin)).responder()

}