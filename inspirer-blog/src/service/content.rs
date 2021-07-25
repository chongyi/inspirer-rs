use inspirer_actix_ext::service::{IntoService, DependencyFactory};
use sqlx::MySqlPool;
use inspirer_content_common::dao::content::{ContentQueryCondition, Key, BindContentToContentEntity, BindSource, ContentEntityByKey};
use inspirer_actix_ext::database::{Get, DAO, CreateDAO, UpdateDAO, DeleteDAO};
use inspirer_content_common::model::content::{ContentBasic, NewContentEntity, NewContent, NewContentEntityWithContent, NewContentMeta, ContentForClient, ContentForClientBasic};
use inspirer_actix_ext::database::statement::pagination::Paginated;
use anyhow::Result;
use crate::request::content::CreateContent;
use crate::error::RuntimeError;

#[derive(Service, FromRequest)]
pub struct ContentService {
    pool: MySqlPool,
}

impl ContentService {
    pub async fn find_basic(&self, key: Key) -> Result<ContentBasic> {
        Get::<ContentBasic>::by(key)
            .run(&self.pool)
            .await
            .map_err(Into::into)
    }

    pub async fn find(&self, key: Key) -> Result<ContentForClient> {
        Get::<ContentForClient>::by(key)
            .run(&self.pool)
            .await
            .map_err(Into::into)
    }

    pub async fn list(&self, query_condition: ContentQueryCondition) -> Result<Paginated<ContentBasic>> {
        Get::<ContentBasic>::by(query_condition)
            .run(&self.pool)
            .await
            .map_err(Into::into)
    }

    pub async fn list_for_client(&self, query_condition: ContentQueryCondition) -> Result<Paginated<ContentForClientBasic>> {
        Get::<ContentForClientBasic>::by(query_condition)
            .run(&self.pool)
            .await
            .map_err(Into::into)
    }

    pub async fn create_content_simple(&self, creator_id: u64, content: &CreateContent) -> Result<u64> {
        let new_content = NewContent {
            creator_id,
            title: content.title.as_str(),
            keywords: content.keywords.as_str(),
            description: content.description.as_str(),
            content_entity_id: 0,
            is_display: false,
        };

        let mut transaction = self.pool.begin().await?;
        let content_id = new_content.create(&mut transaction)
            .await?;

        let content_entity_id = NewContentEntityWithContent {
            content_id,
            entity: NewContentEntity {
                is_draft: content.draft,
                creator_id,
                title: new_content.title,
                keywords: new_content.keywords,
                description: new_content.description,
                content: content.content.as_str(),
            },
        }.create(&mut transaction).await?;

        let result = BindContentToContentEntity(BindSource { content_id, content_entity_id })
            .run(&mut transaction)
            .await?;

        if result {
            transaction.commit().await?;
            Ok(content_id)
        } else {
            transaction.rollback().await?;
            Err(RuntimeError::CreateContentFailed)?
        }
    }

    pub async fn update_content_simple(&self, creator_id: u64, content_id: u64, content: &CreateContent) -> Result<bool> {
        let new_entity = NewContentEntityWithContent {
            content_id,
            entity: NewContentEntity {
                is_draft: content.draft,
                creator_id,
                title: content.title.as_str(),
                keywords: content.keywords.as_str(),
                description: content.description.as_str(),
                content: content.content.as_str(),
            },
        };

        let mut transaction = self.pool.begin().await?;
        let content_entity_id = new_entity.create(&mut transaction)
            .await?;

        let new_meta = NewContentMeta {
            id: content_id,
            content_entity_id,
            title: new_entity.entity.title,
            keywords: new_entity.entity.keywords,
            description: new_entity.entity.description,
        };

        if new_meta.update(&mut transaction)
            .await? {
            transaction.commit().await?;
            Ok(true)
        } else {
            transaction.rollback().await?;
            Ok(false)
        }
    }

    pub async fn delete_content_simple(&self, content_id: u64) -> Result<u64> {
        Ok(
            Key::Id(content_id)
                .delete(&self.pool)
                .await?
        )
    }

    pub async fn force_delete_content_simple(&self, content_id: u64) -> Result<u64> {
        let mut transaction = self.pool.begin().await?;
        let key = Key::Id(content_id);

        key.force_delete(&mut transaction)
            .await?;

        let result = ContentEntityByKey(key)
            .delete(&mut transaction)
            .await?;

        transaction.commit().await?;

        Ok(result)
    }
}