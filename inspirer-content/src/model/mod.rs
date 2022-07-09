use serde::Deserialize;

pub mod content;
pub mod paginate;
pub mod user;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case", tag = "mode", content = "field")]
pub enum Order<T> {
    Asc(T),
    Desc(T),
}

impl<T: Clone> Into<sea_orm::query::Order> for Order<T> {
    fn into(self) -> sea_orm::query::Order {
        self.into_order()
    }
}

impl<T: Clone> Order<T> {
    pub fn inner(&self) -> T {
        match self {
            Order::Asc(inner) => inner.clone(),
            Order::Desc(inner) =>  inner.clone(),
        }
    }

    pub fn into_order(&self) -> sea_orm::query::Order {
        match self {
            Order::Asc(_) => sea_orm::query::Order::Asc,
            Order::Desc(_) => sea_orm::query::Order::Desc,
        }
    }
}
