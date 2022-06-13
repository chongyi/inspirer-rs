use sea_orm::{DbErr, TransactionError};

pub type InspirerContentResult<T, E = Error> = Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    DatabaseError(DbErr),
    #[error("已存在内容")]
    DatabaseWriteConflict,
    #[error("格式化内容错误：{0}")]
    FormatError(serde_json::Error),
    #[error("内容未找到")]
    ContentNotFound,
    #[error("创建内容序列失败")]
    GenerateIdError(#[from] uuid::Error),
    #[error("ID格式非法")]
    ConvertIdError,
    #[error(transparent)]
    RingUnspecifiedError(#[from] ring::error::Unspecified),
    #[error("密钥格式化错误")]
    RingKeyPairFormatError,
    #[error(transparent)]
    PasswordHashError(#[from] argon2::password_hash::Error),
    #[error("用户不存在或密码错误")]
    UserNotFoundOrPasswordError,
}

impl From<DbErr> for Error {
    fn from(err: DbErr) -> Self {
        match err {
            DbErr::Exec(err) if err.contains("1062") && err.contains("23000") => {
                Error::DatabaseWriteConflict
            }
            _ => Error::DatabaseError(err),
        }
    }
}

impl<E> From<TransactionError<E>> for Error
where
    E: Into<Error> + std::error::Error,
{
    fn from(err: TransactionError<E>) -> Self {
        match err {
            TransactionError::Connection(err) => err.into(),
            TransactionError::Transaction(err) => err.into(),
        }
    }
}
