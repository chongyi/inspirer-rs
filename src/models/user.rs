use actix::*;
use actix_web::*;
use diesel::*;
use chrono::NaiveDateTime;

use database::{DatabaseExecutor, Conn};
use util::error::ApplicationError as Error;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct AuthenticationUser {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct UserDisplay {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub struct User;

impl User {
    pub fn find_auth_info_by_email(conn: &Conn, r: String) -> Result<AuthenticationUser, Error> {
        use schema::users::dsl::*;

        Ok(
            users
                .select((id, name, email, password))
                .filter(email.eq(r))
                .first::<AuthenticationUser>(conn)
                .map_err(map_database_error!("users"))?
        )
    }

    pub fn find_user_by_id(conn: &Conn, c: u32) -> Result<UserDisplay, Error> {
        use schema::users::dsl::*;

        Ok(
            users
                .select((id, name, email, created_at, updated_at))
                .filter(id.eq(c))
                .first::<UserDisplay>(conn)
                .map_err(map_database_error!("users"))?
        )
    }

    pub fn find_user_by_email(conn: &Conn, c: String) -> Result<UserDisplay, Error> {
        use schema::users::dsl::*;

        Ok(
            users
                .select((id, name, email, created_at, updated_at))
                .filter(email.eq(c))
                .first::<UserDisplay>(conn)
                .map_err(map_database_error!("users"))?
        )
    }
}

pub struct FindUser {
    pub id: Option<u32>,
    pub email: Option<String>,
}

impl Message for FindUser {
    type Result = Result<UserDisplay, Error>;
}

impl Handler<FindUser> for DatabaseExecutor {
    type Result = Result<UserDisplay, Error>;

    fn handle(&mut self, condition: FindUser, _: &mut Self::Context) -> Self::Result {
        if let Some(id) = condition.id {
            User::find_user_by_id(&self.connection()?, id)
        } else if let Some(email) = condition.email {
            User::find_user_by_email(&self.connection()?, email)
        } else {
            Err(Error::SysLogicArgumentError())
        }
    }
}