pub mod content;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Paginator {
    pub page: u64,
    pub per_page: u64,
}

impl Default for Paginator {
    fn default() -> Self {
        Paginator {
            page: 1,
            per_page: 20,
        }
    }
}

impl Paginator {
    pub fn validated(mut self) -> Self {
        if self.per_page == 0 {
            self.per_page = 20;
        }

        if self.page == 0 {
            self.page = 1;
        }

        self
    }

    pub fn skip(&self) -> u64 {
        self.per_page * (self.page - 1)
    }

    pub fn take(&self) -> u64 {
        self.per_page
    }

    pub fn pagination(self, total: u64) -> Pagination {
        Pagination::new(total, self)
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Pagination {
    pub page: u64,
    pub per_page: u64,
    pub total: u64,
    pub last_page: u64,
}

impl Pagination {
    pub fn new(total: u64, paginator: Paginator) -> Self {
        let paginator = paginator.validated();
        if total == 0 {
            Pagination {
                page: 1,
                per_page: paginator.per_page,
                total,
                last_page: 1,
            }
        } else {
            let last_page = (total as f64 / paginator.per_page as f64).ceil() as u64;
            Pagination {
                page: paginator.page,
                per_page: paginator.per_page,
                total,
                last_page,
            }
        }
    }

    pub fn from_origin_list<T>(origin_list: Vec<(T, u64)>, paginator: Option<Paginator>) -> (Vec<T>, Option<Pagination>) {
        let pagination = paginator.map(|paginator| if origin_list.len() > 0 {
            Pagination::new(origin_list[0].1, paginator)
        } else {
            Pagination::new(0, paginator)
        });

        (origin_list.into_iter().map(|r| r.0).collect(), pagination)
    }
}