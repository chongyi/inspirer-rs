use actix::prelude::*;
use actix_web::*;
use actix_web::middleware::session::SessionBackend;
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
use diesel::MysqlConnection;
use diesel::r2d2::{PooledConnection, ConnectionManager};
use djangohashers::check_password;
use futures::future::{err as FutErr, ok as FutOk, FutureResult};
use futures::Future;
use biscuit::*;
use biscuit::jws::*;
use biscuit::jwa::*;
use chrono::{Utc, Duration};
use dotenv;

use models::user::{User, AuthenticationUser};
use database::DatabaseExecutor;

type Connection = PooledConnection<ConnectionManager<MysqlConnection>>;
type AuthenticateResult = Result<AuthenticationUser, Error>;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PrivateClaims {
    pub uid: u32,
    pub name: String,
}

impl PrivateClaims {
    pub fn generate_jwt_token(&self) -> Result<String, Error> {
        let claims = ClaimsSet::<PrivateClaims> {
            registered: RegisteredClaims {
                expiry: Some(From::from(Utc::now() + Duration::days(1))),
                ..Default::default()
            },
            private: PrivateClaims {
                uid: self.uid.clone(),
                name: self.name.clone(),
            }
        };

        let jwt = JWT::new_decoded(From::from(RegisteredHeader {
            algorithm: SignatureAlgorithm::HS256,
            ..Default::default()
        }), claims.clone());

        let secret = Secret::Bytes(dotenv::var("TOKEN_SECRET").map_err(error::ErrorInternalServerError)?.into_bytes());
        let token = jwt.into_encoded(&secret).map_err(error::ErrorInternalServerError)?;
        let token = token.unwrap_encoded().to_string();

        Ok(token)
    }
}

#[derive(Fail, Debug, PartialEq)]
pub enum AuthenticateError {
    #[fail(display = "Authentication invalidate")]
    ValidateError
}

pub trait Authenticate {
    fn check(&self, result: AuthenticationUser) -> bool;
    fn validate(&self, connection: &Connection) -> AuthenticateResult;
}

pub struct Authentication<T> {
    authentication: T,
}

pub struct Email {
    pub email: String,
    pub password: String,
}

impl<T> Authentication<T> {
    pub fn new(authentication: T) -> Self {
        Authentication {
            authentication,
        }
    }
}

impl Authenticate for Authentication<Email> {
    fn check(&self, result: AuthenticationUser) -> bool {
        if let None = result.password {
            return false;
        }

        check_password(
            self.authentication.password.as_str(),
            result.password.unwrap().as_str()
        ).unwrap_or(false)
    }

    fn validate(&self, connection: &Connection) -> AuthenticateResult {
        let result = User::find_auth_info_by_email(connection, self.authentication.email.clone())?;

        if self.check(result.clone()) {
            return Ok(result);
        }

        Err(error::ErrorForbidden(AuthenticateError::ValidateError))
    }
}

macro_rules! authentication_handler {
    ($x:ident) => {
        impl Message for Authentication<$x> {
            type Result = AuthenticateResult;
        }

        impl Handler<Authentication<$x>> for DatabaseExecutor {
            type Result = AuthenticateResult;

            fn handle(&mut self, auth: Authentication<$x>, _: &mut Self::Context) -> Self::Result {
                Ok(auth.validate(&self.connection()?)?)
            }
        }
    }
}

authentication_handler!(Email);

//pub struct JWTSession {
//    token: Option<PrivateClaims>,
//}
//
//impl JWTSession {
//
//}
//
//pub struct JWTSessionBackend(JWTSession);
//
//impl<S> SessionBackend<S> for JWTSessionBackend {
//    type Session = JWTSession;
//    type ReadFuture = FutureResult<JWTSession, Error>;
//
//    fn from_request(&self, req: &mut HttpRequest<S>) -> Self::ReadFuture {
//        let auth = Authorization::<Bearer>::parse(&req)?;
//
//    }
//}