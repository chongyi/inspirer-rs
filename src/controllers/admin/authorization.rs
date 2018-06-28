use actix_web::*;
use futures::Future;
use biscuit::*;
use biscuit::jws::*;
use biscuit::jwa::*;
use chrono::{Utc, Duration};
use dotenv;

use state::AppState;
use util::auth::Authentication as Auth;
use util::auth::Email;
use util::auth::Authenticate;

#[derive(Deserialize)]
pub struct Authentication {
    email: String,
    password: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PrivateClaims {
    uid: u32,
    name: String,
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
        match res {
            Ok(user) => {
                let claims = ClaimsSet::<PrivateClaims> {
                    registered: RegisteredClaims {
                        expiry: Some(From::from(Utc::now() + Duration::days(1))),
                        ..Default::default()
                    },
                    private: PrivateClaims {
                        uid: user.id,
                        name: user.name,
                    }
                };

                let jwt = JWT::new_decoded(From::from(RegisteredHeader {
                    algorithm: SignatureAlgorithm::HS256,
                    ..Default::default()
                }), claims.clone());

                let secret = Secret::Bytes(dotenv::var("TOKEN_SECRET").map_err(error::ErrorInternalServerError)?.into_bytes());
                let token = jwt.into_encoded(&secret).map_err(error::ErrorInternalServerError)?;

                Ok(HttpResponse::Ok().body(token.unwrap_encoded().to_string()))
            },
            Err(e) => Err(e),
        }

    }).responder()

}