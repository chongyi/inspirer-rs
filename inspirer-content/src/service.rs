use crate::ContentService;
use sqlx::MySqlPool;
use crate::model::{ContentEntityWritable, NewContent, NewContentEntity};
use inspirer_query_ext::dao::{CreateDAO, DAO};
use crate::dao::BindContentToContentEntity;

#[async_trait]
impl ContentService for MySqlPool {
    async fn create(&self, author_id: u64, entity: ContentEntityWritable<'_>) -> anyhow::Result<u64> {
        let mut conn = self.begin().await?;

        let content_id = NewContent { author_id }
            .create(&mut conn)
            .await?;

        let new_content_entity = NewContentEntity {
            author_id,
            content_id,
            previous_id: 0,
            is_draft: false,
            entity,
        };

        let content_entity_id = new_content_entity.create(&mut conn).await?;
        BindContentToContentEntity {
            content_id,
            content_entity_id,
        }.run(&mut conn).await?;

        conn.commit().await?;
        Ok(content_id)
    }

    async fn save_draft(&self, author_id: u64, content_id: u64, entity: ContentEntityWritable<'_>) -> anyhow::Result<u64> {
        todo!()
    }
}