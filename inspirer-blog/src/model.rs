use std::ops::{Deref, DerefMut};

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

#[derive(Serialize, Debug)]
pub struct Paginate<T> {
    data: Vec<T>,
    #[serde(flatten)]
    pagination: Option<Pagination>
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
}

pub trait IntoPaginate<T> {
    fn raw_into(self, paginator: Option<Paginator>) -> Paginate<T>;
}

impl<T> IntoPaginate<T> for Vec<(T, i64)> {
    fn raw_into(self, paginator: Option<Paginator>) -> Paginate<T> {
        let pagination = paginator.map(|paginator| if self.is_empty() {
            Pagination::new(0, paginator)
        } else {
            Pagination::new(self[0].1 as u64, paginator)
        });

        Paginate {
            data: self.into_iter().map(|r| r.0).collect(),
            pagination
        }
    }
}

#[derive(Serialize, Deserialize, Debug, AsRefStr)]
#[serde(rename_all = "snake_case")]
pub enum SortMode {
    #[strum(serialize = "asc")]
    Asc,
    #[strum(serialize = "desc")]
    Desc,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SortOption<T: AsRef<str>> {
    pub field: T,
    pub mode: SortMode,
}

impl<T: AsRef<str>> SortOption<T> {
    pub fn asc(field: T) -> Self {
        SortOption {
            field,
            mode: SortMode::Asc
        }
    }

    pub fn desc(field: T) -> Self {
        SortOption {
            field,
            mode: SortMode::Desc
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SortCondition<T: AsRef<str>> (pub Vec<SortOption<T>>);

impl<T: AsRef<str>> Default for SortCondition<T> {
    fn default() -> Self {
        SortCondition(vec![])
    }
}

impl<T: AsRef<str>> SortCondition<T> {
    pub fn statement(&self) -> String {
        (!self.0.is_empty())
            .then(|| format!(
                "order by {}",
                self.0.iter()
                    .map(|option| format!("{} {}", option.field.as_ref(), option.mode.as_ref()))
                    .collect::<Vec<String>>()
                    .join(",")
            ))
            .unwrap_or(String::new())
    }
}

impl<T: AsRef<str>> Deref for SortCondition<T> {
    type Target = Vec<SortOption<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: AsRef<str>> DerefMut for SortCondition<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}