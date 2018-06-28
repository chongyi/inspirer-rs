use actix::prelude::*;
use actix_web::*;
use diesel::MysqlConnection;
use diesel::r2d2::{PooledConnection, ConnectionManager};
use djangohashers::check_password;

use models::user::User;
use database::DatabaseExecutor;

type Connection = PooledConnection<ConnectionManager<MysqlConnection>>;
type AuthenticateResult = Result<bool, Error>;

pub trait Authenticate {
    fn check(&self, connection: &Connection) -> bool;
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
    fn check(&self, connection: &Connection) -> bool {
        let result = User::find_auth_info_by_email(connection, self.authentication.email.clone());

        match result {
            Ok(auth) => {
                if let None = auth.password {
                    return false;
                }

                self.authentication.password == auth.password.unwrap()
//                check_password(
//                    self.authentication.password.as_str(),
//                    auth.password.unwrap().as_str()
//                ).unwrap_or(false)
            },
            Err(_) => false
        }
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
                Ok(auth.check(&self.connection()?))
            }
        }
    }
}

authentication_handler!(Email);