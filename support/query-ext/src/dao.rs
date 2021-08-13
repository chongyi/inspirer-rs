use sqlx::{Database, Executor};
use std::marker::PhantomData;

/// 基础数据访问对象
///
/// 实现该 Trait 的对象都可以作为数据访问对象
#[async_trait]
pub trait DAO<D: Database> {
    type Result;

    async fn run<'a, E>(&self, executor: E) -> Self::Result
        where E: Executor<'a, Database=D>
    ;
}

/// 数据写入对象
///
/// 实现该 Trait，则可用于写入数据
#[async_trait]
pub trait CreateDAO<D: Database> {
    type Result;

    async fn create<'a, E>(&self, executor: E) -> Self::Result
        where E: Executor<'a, Database=D>
    ;
}


/// 数据读取对象
///
/// 实现该 Trait，则可用于读取数据。不同于其他访问对象，读取数据的访问对象存在第二个需要提供的类型。
///
/// 该 Trait 是实现于查询条件构成的结构体上，第二个需要提供的类型用于声明该查询或读取对象查询的目标。
/// 例如：
///
/// ```ignore
/// impl ReadDAO<MySql, Content> for QueryCondition {
///     type Result = sqlx::Result<Content>;
///
///     async fn read<'a, E>(&self, executor: E) -> Self::Result
///         where E: Executor<'a, Database=MySql> {
///         unimplemented!()
///     }
/// }
/// ```
#[async_trait]
pub trait ReadDAO<D: Database, T> {
    type Result;

    async fn read<'a, E>(&self, executor: E) -> Self::Result
        where E: Executor<'a, Database=D>
    ;
}

/// 数据更新对象
///
/// 实现该 Trait，则可用于数据更新
#[async_trait]
pub trait UpdateDAO<D: Database> {
    type Result;

    async fn update<'a, E>(&self, executor: E) -> Self::Result
        where E: Executor<'a, Database=D>
    ;
}

/// 数据删除对象
///
/// 实现该 Trait，则可用于数据删除
#[async_trait]
pub trait DeleteDAO<D: Database> {
    type Result;

    async fn delete<'a, E>(&self, executor: E) -> Self::Result
        where E: Executor<'a, Database=D>
    ;

    /// 强制删除
    ///
    /// 对于存在软删除的情形，该方法用于强制执行删除
    async fn force_delete<'a, E>(&self, executor: E) -> Self::Result
        where E: Executor<'a, Database=D>
    {
        self.delete(executor).await
    }
}

/// 创建原语结构
pub struct Create<T> (pub T);

#[async_trait]
impl<T, D> DAO<D> for Create<T>
    where T: Sync + Send + CreateDAO<D>,
          D: Database
{
    type Result = T::Result;

    async fn run<'a, E>(&self, executor: E) -> Self::Result
        where E: Executor<'a, Database=D>
    {
        self.0.create(executor).await
    }
}

/// 删除原语结构
pub struct Delete<T> (pub T);

#[async_trait]
impl<T, D> DAO<D> for Delete<T>
    where T: Sync + Send + DeleteDAO<D>,
          D: Database
{
    type Result = T::Result;

    async fn run<'a, E>(&self, executor: E) -> Self::Result
        where E: Executor<'a, Database=D>
    {
        self.0.delete(executor).await
    }
}

/// 强删除原语结构
pub struct ForceDelete<T> (pub T);

#[async_trait]
impl<T, D> DAO<D> for ForceDelete<T>
    where T: Sync + Send + DeleteDAO<D>,
          D: Database
{
    type Result = T::Result;

    async fn run<'a, E>(&self, executor: E) -> Self::Result
        where E: Executor<'a, Database=D>
    {
        self.0.force_delete(executor).await
    }
}

/// 更新原语结构
pub struct Update<T> (pub T);

#[async_trait]
impl<T, D> DAO<D> for Update<T>
    where T: Sync + Send + UpdateDAO<D>,
          D: Database
{
    type Result = T::Result;

    async fn run<'a, E>(&self, executor: E) -> Self::Result
        where E: Executor<'a, Database=D>
    {
        self.0.update(executor).await
    }
}

/// 读取原语结构
pub struct Get<T> (PhantomData<T>);

#[async_trait]
impl<T, C, D> DAO<D> for GetBy<T, C>
    where C: Sync + Send + ReadDAO<D, T>,
          T: Sync + Send,
          D: Database
{
    type Result = C::Result;

    async fn run<'a, E>(&self, executor: E) -> Self::Result
        where E: Executor<'a, Database=D>
    {
        self.condition.read(executor).await
    }
}

/// 条件查询原语结构
pub struct GetBy<T, C> {
    condition: C,
    _target: PhantomData<T>
}

impl<T> Get<T> {
    /// 条件读取
    ///
    /// ```ignore
    /// Get::<Target>::by(condition).run(executor).await;
    /// ```
    pub fn by<C>(condition: C) -> GetBy<T, C> {
        GetBy {
            condition,
            _target: PhantomData
        }
    }
}