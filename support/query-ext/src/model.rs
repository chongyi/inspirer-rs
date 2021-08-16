use serde::de::DeserializeOwned;
use serde::Serialize;
use std::ops::Deref;
use sqlx::{FromRow, Error, Row};
use sqlx::mysql::MySqlRow;

#[derive(Deserialize, Debug)]
pub struct PaginateWrapper<Q> {
    #[serde(flatten)]
    pub paginate: Paginate,
    #[serde(flatten)]
    pub query: Q,
}

impl<Q> PaginateWrapper<Q> {
    pub fn skip(&self) -> u64 {
        self.paginate.per_page * (self.paginate.page - 1)
    }

    pub fn take(&self) -> u64 {
        self.paginate.per_page
    }
}

impl<Q> Deref for PaginateWrapper<Q> {
    type Target = Q;

    fn deref(&self) -> &Self::Target {
        &self.query
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Paginate {
    pub page: u64,
    pub per_page: u64,
}

impl Paginate {
    pub fn wrapped_pagination<T>(&self, raw_paginations: Vec<RawPaginationWrapper<T>>) -> PaginationWrapper<Vec<T>>
        where T: Serialize
    {
        let total = raw_paginations.first()
            .map(|row| row.total as u64)
            .unwrap_or(0);

        let last_page = (total as f64 / self.per_page as f64).ceil() as u64;
        let pagination = Pagination {
            total,
            last_page,
            page: self.page,
            per_page: self.per_page,
        };

        PaginationWrapper {
            data: raw_paginations.into_iter().map(|r| r.data).collect(),
            pagination,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct PaginationWrapper<T: Serialize> {
    #[serde(flatten)]
    pagination: Pagination,
    data: T,
}

#[derive(Serialize, Debug)]
pub struct RawPaginationWrapper<T: Serialize> {
    data: T,
    total: i64,
}

impl<'r, T> FromRow<'r, MySqlRow> for RawPaginationWrapper<T>
    where T: Serialize + FromRow<'r, MySqlRow>
{
    fn from_row(row: &'r MySqlRow) -> Result<Self, Error> {
        Ok(RawPaginationWrapper {
            total: row.try_get("total")?,
            data: T::from_row(row)?,
        })
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct Pagination {
    pub total: u64,
    pub last_page: u64,
    pub page: u64,
    pub per_page: u64,
}