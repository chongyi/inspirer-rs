pub mod mysql;

/// 转换成语句
pub trait IntoStatement<T> {
    fn statement(&self) -> String;
    fn full_statement(&self) -> String;
}