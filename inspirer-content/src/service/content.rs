use sea_orm::TransactionTrait;
use uuid::Uuid;

use crate::{
    dao::content::ContentDao,
    entity::{content_entities, contents},
    error::{Error, InspirerContentResult},
    manager::Manager,
    model::{
        content::{Content, GetListCondition, NewContent},
        paginate::{Paginated, Pagination},
    },
    util::uuid::generate_v1_uuid,
};

#[async_trait::async_trait]
pub trait ContentService {
    async fn get_list(
        &self,
        condition: GetListCondition,
        pagination: Pagination,
    ) -> InspirerContentResult<Paginated<contents::Model>>;
    async fn find_content_by_id(&self, id: Uuid) -> InspirerContentResult<Content>;
    async fn find_content_by_name(&self, name: String) -> InspirerContentResult<Content>;
    async fn create_content(
        &self,
        owner_id: Uuid,
        new_content: NewContent,
    ) -> InspirerContentResult<Uuid>;
}

#[async_trait::async_trait]
impl ContentService for Manager {
    async fn get_list(
        &self,
        condition: GetListCondition,
        pagination: Pagination,
    ) -> InspirerContentResult<Paginated<contents::Model>> {
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
    ) -> InspirerContentResult<Uuid> {
        let id = generate_v1_uuid()?;

        self.database
            .transaction::<_, (), Error>(|trx| {
                Box::pin(async move {
                    trx.create_content(id, owner_id, &new_content).await?;
                    trx.create_content_entity(id, &new_content).await?;
                    Ok(())
                })
            })
            .await?;

        Ok(id)
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
