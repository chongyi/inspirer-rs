use actix_web::*;
use diesel::*;
use diesel::MysqlConnection;
use diesel::r2d2::{PooledConnection, ConnectionManager};

type Conn = PooledConnection<ConnectionManager<MysqlConnection>>;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct AuthenticationUser {
    pub id: u32,
    pub email: String,
    pub password: Option<String>,
}

pub struct User {
    pub id: u32,
    pub email: String,
    pub password: Option<String>,
    pub name: String,
}

impl User {
    pub fn find_auth_info_by_email(conn: &Conn, r: String) -> Result<AuthenticationUser, Error> {
        use schema::users::dsl::*;

        Ok(
            users
                .select((id, email, password))
                .filter(email.eq(r))
                .first::<AuthenticationUser>(conn)
                .map_err(error::ErrorInternalServerError)?
        )
    }
}