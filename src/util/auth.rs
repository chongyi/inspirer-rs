use std::collections::HashMap;

use actix::prelude::*;
use actix_web::*;
use actix_web::Error as ActixError;
use actix_web::http::header::Header;
use actix_web::middleware::Response;
use actix_web::middleware::session::{SessionBackend, SessionImpl};
use actix_web_httpauth::headers::authorization::{Authorization as ActixAuthorization, Bearer};
use djangohashers::check_password;
use futures::future::{ok as FutOk, FutureResult};
use biscuit::*;
use biscuit::Empty as JWTEmpty;
use biscuit::jws::*;
use biscuit::jwa::*;
use chrono::{Utc, Duration};
use dotenv;
use serde_json;

use models::user::{User, AuthenticationUser};
use database::{DatabaseExecutor, Conn as Connection};
use util::error::{ApplicationError as Error};

type AuthenticateResult = Result<AuthenticationUser, Error>;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PrivateClaims {
    pub uid: u32,
    pub name: String,
}

impl PrivateClaims {
    pub fn generate_jwt_token(&self) -> Result<(String, i64), Error> {
        let expired = Utc::now() + Duration::days(1);
        let timestamp = expired.timestamp();
        let claims = ClaimsSet::<PrivateClaims> {
            registered: RegisteredClaims {
                expiry: Some(From::from(expired)),
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

        let secret = Secret::Bytes(dotenv::var("TOKEN_SECRET").or(Err(Error::SysLogicArgumentError()))?.into_bytes());
        let token = jwt.into_encoded(&secret).or(Err(Error::SysLogicArgumentError()))?;
        let token = token.unwrap_encoded().to_string();

        Ok((token, timestamp))
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

        Err(Error::AuthValidationError())
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

pub struct JWTSession {
    pub state: HashMap<String, String>,
}

impl SessionImpl for JWTSession {
    fn get(&self, key: &str) -> Option<&str> {
        if let Some(s) = self.state.get(key) {
            Some(s)
        } else {
            None
        }
    }

    fn set(&mut self, key: &str, value: String) {
        self.state.insert(key.to_owned(), value);
    }

    fn remove(&mut self, key: &str) {
        self.state.remove(key);
    }

    fn clear(&mut self) {
        self.state.clear()
    }

    fn write(&self, resp: HttpResponse) -> Result<Response> {
        Ok(Response::Done(resp))
    }
}

pub struct JWTSessionBackend;

impl JWTSessionBackend {
    pub fn parse_token<S>(&self, req: &mut HttpRequest<S>) -> Result<(PrivateClaims, RegisteredClaims), Error> {
        let auth = ActixAuthorization::<Bearer>::parse(req).or(Err(Error::SysLogicArgumentError()))?;
        let token = auth.token.clone();

        let secret = Secret::Bytes(
            dotenv::var("TOKEN_SECRET")
                .or(Err(Error::SysLogicArgumentError()))?
                .into_bytes()
        );

        let wd = JWT::<PrivateClaims, JWTEmpty>::new_encoded(&token);
        let token = wd.into_decoded(&secret, SignatureAlgorithm::HS256).or(Err(Error::AuthValidationError()))?;

        let payload = token.payload().or(Err(Error::SysLogicArgumentError()))?;
        let claims = (*payload).private.clone();
        let registered = (*payload).registered.clone();

        Ok((claims, registered))
    }
}

impl<S> SessionBackend<S> for JWTSessionBackend {
    type Session = JWTSession;
    type ReadFuture = FutureResult<JWTSession, ActixError>;

    fn from_request(&self, req: &mut HttpRequest<S>) -> Self::ReadFuture {
        let claims = self.parse_token(req);
        match claims {
            Ok(real) => {
                let (claims, registered) = real;
                let mut hash_map: HashMap<String, String> = HashMap::new();
                hash_map.insert("claims".to_owned(), serde_json::to_string(&claims).unwrap());
                hash_map.insert("registered".to_owned(), serde_json::to_string(&registered).unwrap());
                FutOk(JWTSession {
                    state: hash_map,
                })
            },
            Err(_) => FutOk(JWTSession {
                state: HashMap::new(),
            })
        }

    }
}