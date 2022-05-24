use sea_orm::DbErr;

pub type InspirerContentResult<T, E = Error> = Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    DatabaseError(DbErr),
    #[error("已存在内容")]
    DatabaseWriteConflict,
    #[error("格式化内容错误：{0}")]
    FormatError(serde_json::Error),
}

impl From<DbErr> for Error {
    fn from(err: DbErr) -> Self {
        match err {
            DbErr::Exec(err) if err.contains("1062") && err.contains("23000") => Error::DatabaseWriteConflict,
            _ => Error::DatabaseError(err)
        }
    }
}