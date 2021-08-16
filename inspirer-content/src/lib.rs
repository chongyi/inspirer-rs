#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate strum;

use crate::model::{ContentEntityWritable, ContentStatusWritable, ContentEntityFull, ContentWithEntity, AdvanceContentQuery, ContentWithEntitySummary};
use anyhow::Result;
use inspirer_query_ext::model::{PaginateWrapper, PaginationWrapper};

pub mod dao;
pub mod model;
pub mod error;
pub mod service;

#[async_trait]
pub trait ContentService {
    /// 创建一个内容
    ///
    /// 基于提供的内容实体写入内容，并返回对应的 ID
    async fn create(&self, author_id: u64, entity: ContentEntityWritable<'_>) -> Result<u64>;

    /// 保存或覆盖草稿
    async fn override_draft(&self, author_id: u64, content_id: u64, is_draft: bool, entity: ContentEntityWritable<'_>) -> Result<u64>;

    /// 通过草稿创建内容
    ///
    /// 若创建前需要更新草稿内容，最后一个参数需要提供
    async fn create_from_draft(&self, author_id: u64, draft_id: u64, entity: Option<ContentEntityWritable<'_>>) -> Result<u64>;

    /// 获取最新的草稿内容
    async fn get_latest_draft(&self, content_id: u64) -> Result<Option<ContentEntityFull>>;

    /// 更新内容
    ///
    /// 若存在草稿则会覆盖
    async fn update(&self, author_id: u64, content_id: u64, entity: ContentEntityWritable<'_>) -> Result<bool>;

    /// 删除内容
    ///
    /// 默认为软删除，若强制删除则会清空所有数据，包括与其关联的内容实体
    async fn delete(&self, content_id: u64, force_delete: bool) -> Result<u64>;

    /// 获取内容
    async fn get(&self, content_id: u64) -> Result<Option<ContentWithEntity>>;

    /// 获取内容列表
    async fn list(&self, query: PaginateWrapper<AdvanceContentQuery>) -> Result<PaginationWrapper<Vec<ContentWithEntitySummary>>>;
}