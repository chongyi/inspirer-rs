//! 数据服务代理动态模型对象
//!
//! 基于代理动态模型对象（Agent Active Model），可以剥离常规模型和服务业务逻辑，代理对象是模型的高级封装。
//!
//! `代理动态模型对象` 可以理解为 DAO 层。因为 Rust 零开销抽象的特性，我们不再通过传统的单个 DAO 对象内多个方法。
//! 因为数据结构针对写入和读出两类情况，需要考虑到性能问题采用不同的数据类型（常见于文本的引用），
//! 我们可以通过结构体来进行数据库执行上下文参数的载体，因此这种设计方式最符合 Rust 的思想。
//! 这也是为何叫做 **代理** 而非 **实现**。

pub mod user;
pub mod validate_code;
pub mod content;

use crate::prelude::*;

/// 动态可执行对象模型
///
/// 实现该 Trait，即可基于此即可创建一个具体的代理动态模型对象。该对象可被执行，执行的逻辑往往是一段具体的数据层操作，
/// 该业务针对这个模型特征而定。
///
/// 例如一个 `CreateUser` 模型，对应的逻辑就是创建一个用户的全部过程，中间可能会交叉操作多个数据表（根据实际业务而定）。
pub trait ActiveModel {
    /// 返回的结果类型
    ///
    /// 依据该泛型定义实现对象的执行结果返回类型。
    type Result;

    /// 执行（或称 `激活`）该动态模型
    ///
    /// 实现的具体行为依据动态模型的内部数据，动态模型本身就是一个参数体。
    fn activate(&self, conn: &PooledConn) -> Self::Result;
}

/// 事务数据模型
///
/// 通过该结构包装一个或一系列待执行对象，在执行过程中会以事务的方式进行。
///
/// 若希望在一个事务中执行多个 Active Model 可自行创建一个单独的 Active Model，
/// 并在其中创建并调用你希望执行的那几个 Active Model，再将该 Active Model 置于
/// 事务数据模型内，即可。
///
/// ## 例子
///
/// ```rust
/// use inspirer_data_provider::prelude::*;
/// use inspirer_data_provider::schema::contents::dsl::*;
/// use diesel::sql_types::*;
///
/// let cpm = ConnPoolManager::builder().build();
/// let conn = cpm.pool().get().unwrap();
///
/// struct TestTransaction<'a>(pub bool, pub &'a str, pub &'a str);
///
/// impl<'a> ActiveModel for TestTransaction<'a> {
///     type Result = ActionResult<()>;
///
///     fn activate(&self, conn: &PooledConn) -> Self::Result {
///         diesel::insert_into(contents)
///             .values((creator_uuid.eq(self.1), title.eq(format!("content {}", self.2))))
///             .execute(conn);
///
///         if self.0 {
///             Ok(())
///         } else {
///             Err(diesel::result::Error::NotFound).map_err(From::from)
///         }
///     }
/// }
///
/// let a = Transaction(Box::new(TestTransaction(true, "b9e87a68d0dd4748806e7ddb403701f5", "test title 1")));
/// a.activate(&conn);
/// let b = Transaction(Box::new(TestTransaction(false, "b9e87a68d0dd4748806e7ddb403701f5", "test title 2")));
/// // Rollback
/// b.activate(&conn);
/// let c = Transaction(Box::new(TestTransaction(true, "b9e87a68d0dd4748806e7ddb403701f5", "test title 3")));
/// c.activate(&conn);
/// ```
pub struct Transaction<T>(pub Box<dyn ActiveModel<Result = ActionResult<T>>>);

impl<T> ActiveModel for Transaction<T> {
    type Result = ActionResult<T>;

    fn activate(&self, conn: &PooledConn) -> Self::Result {
        conn.transaction(|| {
            let d = self.0.as_ref();
            d.activate(conn)
        })
    }
}

#[derive(Deserialize)]
pub struct WithId<I, D> {
    pub id: I,
    pub data: D,
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use diesel::sql_types::*;

    #[test]
    fn test_transaction_wrapper() {
        use crate::schema::contents::dsl::*;

        let cpm = ConnPoolManager::builder().build();
        let conn = cpm.pool().get().unwrap();

        struct TestTransaction<'a>(pub bool, pub &'a str, pub &'a str);

        impl<'a> ActiveModel for TestTransaction<'a> {
            type Result = ActionResult<()>;

            fn activate(&self, conn: &PooledConn) -> Self::Result {
                diesel::insert_into(contents)
                    .values((creator_uuid.eq(self.1), title.eq(format!("content {}", self.2))))
                    .execute(conn);

                if self.0 {
                    Ok(())
                } else {
                    Err(diesel::result::Error::NotFound).map_err(From::from)
                }
            }
        }

        let a = Transaction(Box::new(TestTransaction(true, "b9e87a68d0dd4748806e7ddb403701f5", "test title 1")));
        a.activate(&conn);
        let b = Transaction(Box::new(TestTransaction(false, "b9e87a68d0dd4748806e7ddb403701f5", "test title 2")));
        b.activate(&conn);
        let c = Transaction(Box::new(TestTransaction(true, "b9e87a68d0dd4748806e7ddb403701f5", "test title 3")));
        c.activate(&conn);

        let r: Vec<(String, String)> = diesel::dsl::sql::<(VarChar, VarChar)>("select creator_uuid, title from contents order by id").get_results(&conn).unwrap();
        assert_eq!(vec![("b9e87a68d0dd4748806e7ddb403701f5".to_string(), "content test title 1".to_string()), ("b9e87a68d0dd4748806e7ddb403701f5".to_string(), "content test title 3".to_string())], r);
        diesel::sql_query("truncate table contents restart identity").execute(&conn);
    }
}