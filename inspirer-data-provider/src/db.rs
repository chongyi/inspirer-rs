//! 数据库模块

use diesel::r2d2::{self, ConnectionManager, event::*};
use diesel::query_dsl::methods::LoadQuery;
use diesel::query_builder::*;
use diesel::pg::Pg;
use diesel::sql_types::BigInt;
use crate::prelude::*;
use crate::utils;
use std::fmt::{Formatter, Error};

/// 重命名 `PgConnection` 为 `DbConnection`
pub type DbConnection = PgConnection;

/// 重命名 `Pg` 为 `DbType`
pub type DbType = Pg;

/// 数据库连接池管理器对象
#[derive(Clone)]
pub struct ConnPoolManager {
    w: Pool,
    r: Option<Vec<Pool>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ConnectionConfig {
    pub writer: ConfigMeta,
    pub reader: Option<Vec<ConfigMeta>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConfigMeta {
    pub database_url: String,
}

impl Default for ConfigMeta {
    fn default() -> Self {
        ConfigMeta {
            database_url: "postgres://postgres:postgres@127.0.0.1/inspirer".into(),
        }
    }
}

/// 数据库连接元信息
#[derive(Debug, Clone)]
pub struct ConnectionMeta {
    pub mode: ConnectionMode
}

/// 数据库连接模式
#[derive(Debug, Clone)]
pub enum ConnectionMode {
    /// 写库模式
    Writer,
    /// 读库模式，因为允许存在多个读连接，其中包裹的数值是连接记录序号的
    Reader(usize),
}

impl std::fmt::Display for ConnectionMode {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            ConnectionMode::Writer => write!(f, "writer"),
            ConnectionMode::Reader(i) => write!(f, "reader({})", i)
        }
    }
}

impl std::fmt::Display for ConnectionMeta {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self.mode {
            ConnectionMode::Writer => write!(f, "writer"),
            ConnectionMode::Reader(i) => write!(f, "reader({})", i)
        }
    }
}

/// 错误处理器
pub trait ErrorHandler<E>: std::fmt::Debug + Send + Sync + 'static {
    /// 对错误进行处理的具体逻辑。可以通过 `conn_type` 获取当前数据库连接的元信息
    fn handle_error(&self, conn_type: &ConnectionMeta, error: E);
}

pub trait EventHandler: std::fmt::Debug + Send + Sync {
    /// 当连接被捕获.
    #[allow(unused_variables)]
    fn handle_acquire(&self, conn_type: &ConnectionMeta, event: AcquireEvent) {}

    /// 当连接被释放.
    #[allow(unused_variables)]
    fn handle_release(&self, conn_type: &ConnectionMeta, event: ReleaseEvent) {}

    /// 当连接被取出.
    #[allow(unused_variables)]
    fn handle_checkout(&self, conn_type: &ConnectionMeta, event: CheckoutEvent) {}

    /// 当连接超时.
    #[allow(unused_variables)]
    fn handle_timeout(&self, conn_type: &ConnectionMeta, event: TimeoutEvent) {}

    /// 当连接被放入.
    #[allow(unused_variables)]
    fn handle_checkin(&self, conn_type: &ConnectionMeta, event: CheckinEvent) {}
}

/// 错误管理器包装器
///
/// 用于将当前项目自定义的 Error Handler 包装，然后对该包装器实现 `r2d2::HandleError`，
/// 这样的目的是用于实现在错误处理时传递数据库连接的元信息
#[derive(Debug)]
pub struct ErrorHandlerWrapper {
    pub handler: Box<ErrorHandler<<ConnectionManager<DbConnection> as r2d2::ManageConnection>::Error>>,
    pub meta: ConnectionMeta,
}

impl r2d2::HandleError<<ConnectionManager<DbConnection> as r2d2::ManageConnection>::Error> for ErrorHandlerWrapper
{
    fn handle_error(&self, error: <ConnectionManager<DbConnection> as r2d2::ManageConnection>::Error) {
        self.handler.as_ref().handle_error(&self.meta, error)
    }
}

/// 事件管理器包装器
///
/// 用于将当前项目自定义的 Event Handler 包装，然后对该包装器实现 `r2d2::HandleEvent`，
/// 这样的目的是用于实现在事件处理时传递数据库连接的元信息
#[derive(Debug)]
pub struct EventHandlerWrapper {
    pub handler: Box<EventHandler>,
    pub meta: ConnectionMeta,
}

impl r2d2::HandleEvent for EventHandlerWrapper {
    fn handle_acquire(&self, event: AcquireEvent) {
        self.handler.as_ref().handle_acquire(&self.meta, event);
    }

    fn handle_release(&self, event: ReleaseEvent) {
        self.handler.as_ref().handle_release(&self.meta, event);
    }

    fn handle_checkout(&self, event: CheckoutEvent) {
        self.handler.as_ref().handle_checkout(&self.meta, event);
    }

    fn handle_timeout(&self, event: TimeoutEvent) {
        self.handler.as_ref().handle_timeout(&self.meta, event);
    }

    fn handle_checkin(&self, event: CheckinEvent) {
        self.handler.as_ref().handle_checkin(&self.meta, event);
    }
}

#[derive(Debug)]
pub struct LoggingErrorHandlerWrapper;

impl<E> ErrorHandler<E> for LoggingErrorHandlerWrapper
    where E: std::error::Error
{
    fn handle_error(&self, conn_type: &ConnectionMeta, error: E) {
        error!("[{}] {}", conn_type, error);
    }
}

#[derive(Debug)]
pub struct NopEventHandlerWrapper;

impl EventHandler for NopEventHandlerWrapper {}

/// 数据库连接池管理对象构造器
pub struct Builder {
    error_handler: Box<dyn Fn() -> Box<dyn ErrorHandler<<ConnectionManager<DbConnection> as r2d2::ManageConnection>::Error>>>,
    event_handler: Box<dyn Fn() -> Box<dyn EventHandler>>,
    config: ConnectionConfig,
}

impl Builder {
    pub fn error_handler(mut self, handler: Box<dyn Fn() -> Box<dyn ErrorHandler<<ConnectionManager<DbConnection> as r2d2::ManageConnection>::Error>>>) -> Self {
        self.error_handler = handler;
        self
    }

    pub fn event_handler(mut self, handler: Box<dyn Fn() -> Box<dyn EventHandler>>) -> Self {
        self.event_handler = handler;
        self
    }

    pub fn writer(mut self, writer: ConfigMeta) -> Self {
        self.config.writer = writer;
        self
    }

    pub fn readers(mut self, readers: Vec<ConfigMeta>) -> Self {
        self.config.reader = Some(readers);
        self
    }

    pub fn reader(mut self, reader: ConfigMeta) -> Self {
        match self.config.reader {
            Some(ref mut readers) => {
                readers.push(reader);
            }
            None => self.config.reader = Some(vec![reader])
        }
        self
    }

    pub fn build(self) -> ConnPoolManager {
        let error_handler_builder = self.error_handler.as_ref();
        let event_handler_builder = self.event_handler.as_ref();

        // 写库的连接池
        let writer_pool = {
            let error_handler = error_handler_builder();
            let event_handler = event_handler_builder();
            utils::create_pool_builder_with_handler(ConnectionMeta { mode: ConnectionMode::Writer }, error_handler, event_handler)
                .build(ConnectionManager::<DbConnection>::new(self.config.writer.database_url.as_str()))
                .expect("error: Build writer connection pool failed.")
        };

        // 读库的连接池
        let reader_pool = match self.config.reader {
            Some(config) => {
                let mut index = 0;
                let mut pools = vec![];
                for meta in &config {
                    let error_handler = error_handler_builder();
                    let event_handler = event_handler_builder();
                    let pool = utils::create_pool_builder_with_handler(ConnectionMeta { mode: ConnectionMode::Reader(index) }, error_handler, event_handler)
                        .build(ConnectionManager::<DbConnection>::new(meta.database_url.as_str()))
                        .expect(&format!("error: Build reader({}) connection pool failed.", index));

                    pools.push(pool);
                    index = index + 1;
                }
                Some(pools)
            }
            None => None,
        };

        ConnPoolManager::create(writer_pool, reader_pool)
    }
}

impl Default for Builder {
    fn default() -> Self {
        Builder {
            error_handler: Box::new(|| Box::new(LoggingErrorHandlerWrapper)),
            event_handler: Box::new(|| Box::new(NopEventHandlerWrapper)),
            config: ConnectionConfig::default(),
        }
    }
}

impl From<Builder> for ConnPoolManager {
    fn from(builder: Builder) -> Self {
        builder.build()
    }
}

impl ConnPoolManager {
    pub fn builder() -> Builder {
        Builder::default()
    }

    pub fn create(writer: Pool, reader: Option<Vec<Pool>>) -> Self {
        ConnPoolManager {
            w: writer,
            r: reader,
        }
    }

    pub fn pool(&self) -> &Pool {
        &self.w
    }
}

/// 默认分页
pub const DEFAULT_PER_PAGE: i64 = 15;

#[derive(Debug, Clone, Copy, QueryId)]
pub struct Paginated<T> {
    data: T,
    page: i64,
    per_page: i64,
}

pub trait Paginator: Sized {
    fn paginate(self, page: i64) -> Paginated<Self>;
}

impl<T> Paginator for T {
    fn paginate(self, page: i64) -> Paginated<Self> {
        Paginated {
            data: self,
            per_page: DEFAULT_PER_PAGE,
            page
        }
    }
}

impl<T> Paginated<T> {
    pub fn per_page(self, per_page: i64) -> Self {
        Paginated { per_page, ..self }
    }

    /// 加载且计算分页信息
    ///
    /// 返回值为 `(result, total page, total records)`
    pub fn load_and_count_pages<U>(self, conn: &DbConnection) -> QueryResult<(Vec<U>, i64, i64)>
    where
        Self: LoadQuery<DbConnection, (U, i64)>,
    {
        let per_page = self.per_page;
        let results = self.load::<(U, i64)>(conn)?;

        // 获取记录总数
        let total = results.get(0).map(|x| x.1).unwrap_or(0);

        // 获取所有记录
        let records = results.into_iter().map(|x| x.0).collect();

        // 计算页码
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;

        Ok((records, total_pages, total))
    }
}

impl<T: Query> Query for Paginated<T> {
    type SqlType = (T::SqlType, BigInt);
}

impl<T> RunQueryDsl<DbConnection> for Paginated<T> {}

impl<T> QueryFragment<DbType> for Paginated<T>
where
    T: QueryFragment<DbType>,
{
    fn walk_ast(&self, mut out: AstPass<DbType>) -> QueryResult<()> {
        out.push_sql("select *, count(*), over () from (");
        self.data.walk_ast(out.reborrow())?;
        out.push_sql(") t limit ");
        out.push_bind_param::<BigInt, _>(&self.per_page)?;
        out.push_sql(" offset ");
        let offset = (self.page - 1) * self.per_page;
        out.push_bind_param::<BigInt, _>(&offset)?;
        Ok(())
    }
}