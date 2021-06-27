pub mod content;
pub mod user;

pub fn condition_str(conditions: Vec<&str>) -> String {
    (!conditions.is_empty())
        .then(|| {
            format!("where {}", conditions.join(" and "))
        })
        .unwrap_or(String::new())
}