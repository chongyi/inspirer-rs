use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
#[serde(default)]
pub struct Pagination {
    pub page: usize,
    pub page_size: usize,
}

impl Default for Pagination {
    fn default() -> Self {
        Pagination {
            page: 1,
            page_size: 20,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Paginated<T: Serialize> {
    pub data: Vec<T>,
    pub page: usize,
    pub page_size: usize,
    pub total: usize,
    pub last_page: usize,
}

impl<T: Serialize> Paginated<T> {
    pub fn map<U: Serialize, F: FnOnce(Vec<T>) -> Vec<U>>(self, op: F) -> Paginated<U> {
        Paginated {
            data: op(self.data),
            page: self.page,
            page_size: self.page_size,
            total: self.total,
            last_page: self.last_page
        }
    }
}