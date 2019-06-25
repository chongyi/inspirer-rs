#![recursion_limit = "512"]
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

pub mod agent;
pub mod schema;
pub mod db;
pub mod result;
pub mod utils;
pub mod model;

#[cfg(test)]
pub(crate) mod tests;

pub use db::DbConnection;

/// 从连接池获取的可用连接
pub type PooledConn = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<DbConnection>>;
pub type Pool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<DbConnection>>;

pub mod prelude {
    pub use crate::agent::{ActiveModel, Transaction};
    pub use crate::db::{EventHandler, ErrorHandler, ConnPoolManager, ConnectionConfig, ConfigMeta, DbConnection, Paginator, Paginated, DEFAULT_PER_PAGE};
    pub use crate::result::{self, ErrorKind, ActionResult, PaginateWrapper};
    pub use super::{PooledConn, Pool};
    pub use super::utils;
    pub use inspirer_common::result::CodedError;
    pub use diesel::prelude::*;
    #[cfg(test)]
    pub use crate::tests::helper::*;
}
