use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct Manager {
    database: DatabaseConnection
}