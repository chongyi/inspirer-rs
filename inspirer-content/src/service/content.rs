use sea_orm::TransactionTrait;
use strum::VariantNames;
use uuid::Uuid;

use crate::{
    dao::content::{ContentDao, ContentUpdateLogDao},
    entity::{content_entities, contents, users},
    enumerate::content::ContentType,
    error::{Error, InspirerContentResult},
    manager::Manager,
    model::{
        content::{Content, ContentConfig, GetListCondition, NewContent, UpdateContent},
        paginate::{Paginated, Pagination},
    },
    util::uuid::generate_v1_uuid,
};

#[async_trait::async_trait]
pub trait ContentService {
    async fn get_content_service_config(&self) -> InspirerContentResult<ContentConfig>;
    async fn get_list(
        &self,
        condition: GetListCondition,
        pagination: Pagination,
    ) -> InspirerContentResult<Paginated<(contents::Model, Option<users::Model>)>>;
    async fn get_deleted_content_list(
        &self,
        mut condition: GetListCondition,
        pagination: Pagination,
    ) -> InspirerContentResult<Paginated<(contents::Model, Option<users::Model>)>> {
        condition.list_deleted = true;
        self.get_list(condition, pagination).await
    }
    async fn find_content_by_id(&self, id: Uuid) -> InspirerContentResult<Content>;
    async fn find_content_by_name(&self, name: String) -> InspirerContentResult<Content>;
    async fn create_content(
        &self,
        owner_id: Uuid,
        new_content: NewContent,
    ) -> InspirerContentResult<Content>;
    async fn update_content(
        &self,
        user_id: Uuid,
        content_id: Uuid,
        update_content: UpdateContent,
    ) -> InspirerContentResult<()>;
    async fn publish_content(&self, id: Uuid) -> InspirerContentResult<()>;
    async fn unpublish_content(&self, id: Uuid) -> InspirerContentResult<()>;
    async fn delete_content(&self, id: Uuid, force: bool) -> InspirerContentResult<()>;
    async fn revert_deleted_content(&self, id: Uuid) -> InspirerContentResult<()>;
}

#[async_trait::async_trait]
impl ContentService for Manager {
    async fn get_list(
        &self,
        condition: GetListCondition,
        pagination: Pagination,
    ) -> InspirerContentResult<Paginated<(contents::Model, Option<users::Model>)>> {
        self.database.get_list(condition, pagination).await
    }
    async fn find_content_by_id(&self, id: Uuid) -> InspirerContentResult<Content> {
        convert_content(self.database.find_content_by_id(id).await?)
    }
    async fn find_content_by_name(&self, name: String) -> InspirerContentResult<Content> {
        convert_content(self.database.find_content_by_name(name).await?)
    }
    async fn create_content(
        &self,
        owner_id: Uuid,
        new_content: NewContent,
    ) -> InspirerContentResult<Content> {
        let id = generate_v1_uuid()?;
        let update_log_id = generate_v1_uuid()?;

        self.database
            .transaction::<_, (), Error>(|trx| {
                Box::pin(async move {
                    trx.create_content(id, owner_id, &new_content).await?;
                    trx.create_content_entity(id, &new_content).await?;
                    trx.create_content_update_log(update_log_id, owner_id, id, new_content.into())
                        .await?;
                    Ok(())
                })
            })
            .await?;

        self.find_content_by_id(id).await
    }
    async fn update_content(
        &self,
        user_id: Uuid,
        content_id: Uuid,
        update_content: UpdateContent,
    ) -> InspirerContentResult<()> {
        let update_log_id = generate_v1_uuid()?;

        self.database
            .transaction::<_, (), Error>(|trx| {
                Box::pin(async move {
                    trx.update_content(content_id, &update_content).await?;
                    trx.update_content_entity(content_id, &update_content)
                        .await?;
                    trx.create_content_update_log(
                        update_log_id,
                        user_id,
                        content_id,
                        update_content,
                    )
                    .await?;

                    Ok(())
                })
            })
            .await?;

        Ok(())
    }

    async fn get_content_service_config(&self) -> InspirerContentResult<ContentConfig> {
        Ok(ContentConfig {
            content_support_type: ContentType::VARIANTS,
        })
    }

    async fn publish_content(&self, id: Uuid) -> InspirerContentResult<()> {
        self.database.publish_content(id).await
    }

    async fn unpublish_content(&self, id: Uuid) -> InspirerContentResult<()> {
        self.database.unpublish_content(id).await
    }

    async fn delete_content(&self, id: Uuid, force: bool) -> InspirerContentResult<()> {
        if force {
            self.database
                .transaction::<_, (), Error>(|trx| {
                    Box::pin(async move {
                        trx.force_delete_content(id).await?;
                        trx.delete_content_entity(id).await?;
                        Ok(())
                    })
                })
                .await?;
        } else {
            self.database.delete_content(id).await?;
        }

        Ok(())
    }

    async fn revert_deleted_content(&self, id: Uuid) -> InspirerContentResult<()> {
        self.database.revert_deleted_content(id).await
    }
}

fn convert_content(
    res: Option<(contents::Model, Option<content_entities::Model>)>,
) -> InspirerContentResult<Content> {
    res.ok_or(Error::ContentNotFound).map(|(meta, entity)| {
        let entity = entity
            .and_then(|model| {
                serde_json::from_value(model.entity)
                    .map_err(|err| tracing::error!("Format content entity error: {}", err))
                    .ok()
            })
            .unwrap_or_default();
        Content { meta, entity }
    })
}
