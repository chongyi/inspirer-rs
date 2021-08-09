/// 排序选项
#[derive(Serialize, Deserialize, AsRefStr, Debug, Clone)]
#[serde(tag = "mode", content = "column")]
pub enum Sort<T> {
    #[serde(rename = "asc")]
    #[strum(serialize = "asc")]
    Asc(T),
    #[serde(rename = "desc")]
    #[strum(serialize = "desc")]
    Desc(T),
}

/// 排序语句
pub type SortStatement<T> = Vec<Sort<T>>;