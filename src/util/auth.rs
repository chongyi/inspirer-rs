use actix::prelude::*;
use actix_web::*;
use diesel::MysqlConnection;
use diesel::r2d2::{PooledConnection, ConnectionManager};
use djangohashers::check_password;

use models::user::{User, AuthenticationUser};
use database::DatabaseExecutor;

type Connection = PooledConnection<ConnectionManager<MysqlConnection>>;
type AuthenticateResult = Result<AuthenticationUser, Error>;

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