mod mysql;

pub trait IntoStatement<T> {
    fn statement(&self) -> String;
    fn full_statement(&self) -> String;
}