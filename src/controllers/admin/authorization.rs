use actix_web::*;
use futures::Future;

use state::AppState;
use util::auth::{Authentication as Auth, PrivateClaims, Email};
use util::error::runtime_error_container;

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
    let extractor = req.clone();
    Form::<Authentication>::extract(&extractor).from_err().and_then(move |r| {
        let auth = Email {
            email: r.email.clone(),
            password: r.password.clone(),
        };

        let checker = Auth::new(auth);

        extractor.state().database.send(checker).from_err()
    }).and_then(|res| {
        let user = res.map_err(runtime_error_container(req).into())?;
        let claims = PrivateClaims {
            uid: user.id,
            name: user.name,
        };

        let (token, expired) = claims.generate_jwt_token().map_err(runtime_error_container(req).into())?;

        Ok(HttpResponse::Ok().json(AuthorizationResult {
            token,
            exp: expired,
        }))

    }).responder()

}