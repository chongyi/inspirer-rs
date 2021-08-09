#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate strum;

use crate::model::{ContentEntityWritable, ContentStatusWritable, ContentEntityFull};
use anyhow::Result;

pub mod dao;
pub mod model;
pub mod error;
pub mod service;

#[async_trait]
pub trait ContentService {
    /// 创建一个内容
    async fn create(&self, author_id: u64, entity: ContentEntityWritable<'_>) -> Result<u64>;

    /// 保存草稿
    async fn save_draft(&self, author_id: u64, content_id: u64, entity: ContentEntityWritable<'_>) -> Result<u64>;


}