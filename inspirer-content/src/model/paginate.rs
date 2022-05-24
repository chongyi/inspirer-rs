use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
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
pub struct Paginated<T> {
    pub data: Vec<T>,
    pub page: usize,
    pub page_size: usize,
    pub total: usize,
    pub last_page: usize,
}
