use sqlx::{MySqlPool, Acquire};

use inspirer_query_ext::dao::{CreateDAO, DAO, Get, UpdateDAO, ReadDAO, DeleteDAO};

use crate::ContentService;
use crate::model::{ContentEntityMeta, ContentEntityWritable, NewContent, NewContentEntity, UpdateContentEntity, ContentEntityFull, DeleteContent, DeleteContentEntityByContentId};
use crate::model::content::{BindContentToContentEntity, GetLatestContentEntity};
use crate::error::InspirerContentError;

#[async_trait]
impl ContentService for MySqlPool {
    async fn create(&self, author_id: u64, entity: ContentEntityWritable<'_>) -> anyhow::Result<u64> {
        let mut conn = self.begin().await?;

        // 先创建内容
        let content_id = NewContent { author_id }
            .create(&mut conn)
            .await?;

        // 再创建内容实体
        let content_entity_id = NewContentEntity {
            author_id,
            content_id,
            previous_id: 0,
            is_draft: false,
            entity,
        }.create(&mut conn).await?;

        // 绑定内容至内容实体
        BindContentToContentEntity {
            content_id,
            content_entity_id,
        }.run(&mut conn).await?;

        conn.commit().await?;
        Ok(content_id)
    }

    async fn override_draft(&self, author_id: u64, content_id: u64, is_draft: bool, entity: ContentEntityWritable<'_>) -> anyhow::Result<u64> {
        let mut conn = self.begin().await?;

        let latest_content_entity_meta = Get::<ContentEntityMeta>::by(GetLatestContentEntity {content_id, is_draft: true})
            .run(&mut conn)
            .await?;

        let result = match latest_content_entity_meta {
            Some(content_entity_meta) => {
                // 如果存在草稿，则直接覆盖
                // 若不是草稿，则追加草稿内容
                if content_entity_meta.is_draft {
                    UpdateContentEntity {
                        id: content_entity_meta.id,
                        content_id,
                        is_draft,
                        entity,
                    }.update(&mut conn).await?;
                    content_entity_meta.id
                } else {
                    NewContentEntity {
                        author_id,
                        content_id,
                        previous_id: content_entity_meta.id,
                        is_draft,
                        entity,
                    }.create(&mut conn).await?
                }
            }
            None => NewContentEntity {
                author_id,
                content_id,
                previous_id: 0,
                is_draft,
                entity,
            }.create(&mut conn).await?
        };

        conn.commit().await?;
        Ok(result)
    }


    async fn create_from_draft(&self, author_id: u64, draft_id: u64, entity: Option<ContentEntityWritable<'_>>) -> anyhow::Result<u64> {
        let mut conn = self.begin().await?;

        // 先创建内容
        let content_id = NewContent { author_id }
            .create(&mut conn)
            .await?;

        // 更新已存在的草稿内容
        if let Some(entity) = entity {
            let result = UpdateContentEntity {
                id: draft_id,
                content_id,
                is_draft: false,
                entity,
            }.update(&mut conn).await?;

            if !result {
                Err(InspirerContentError::CannotCreateContentFromDraft)?;
            }
        }

        // 绑定内容至内容实体
        BindContentToContentEntity {
            content_id,
            content_entity_id: draft_id,
        }.run(&mut conn).await?;

        conn.commit().await?;

        Ok(content_id)
    }

    async fn get_latest_draft(&self, content_id: u64) -> anyhow::Result<Option<ContentEntityFull>> {
        Get::<ContentEntityFull>::by(GetLatestContentEntity { content_id, is_draft: true })
            .run(self)
            .await
            .map_err(Into::into)
    }

    async fn update(&self, author_id: u64, content_id: u64, entity: ContentEntityWritable<'_>) -> anyhow::Result<bool> {
        self.override_draft(author_id, content_id, false, entity)
            .await
            .map(|_| true)
    }

    async fn delete(&self, content_id: u64, force_delete: bool) -> anyhow::Result<u64> {
        let content_delete_executor = DeleteContent(content_id);

        if force_delete {
            let mut conn = self.begin().await?;

            DeleteContentEntityByContentId(content_id)
                .delete(&mut conn)
                .await?;

            content_delete_executor.force_delete(&mut conn)
                .await
                .map_err(Into::into)
        } else {
            content_delete_executor.delete(self)
                .await
                .map_err(Into::into)
        }
    }
}