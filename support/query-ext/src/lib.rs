#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate strum;

pub mod filter;
pub mod sort;
pub mod statement;
pub mod dao;

pub(crate) mod test {
    #[derive(Serialize, Deserialize, AsRefStr, PartialEq, Debug)]
    #[serde(rename_all = "snake_case")]
    pub enum SortColumn {
        #[strum(serialize = "content.id")]
        Id,
        #[strum(serialize = "content.create_time")]
        CreateTime,
    }
}