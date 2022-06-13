use sea_orm::{
    ColumnTrait, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
};
use uuid::Uuid;

use crate::{
    entity::content_entities,
    entity::contents,
    enumerate::content::ContentType,
    error::InspirerContentResult,
    model::{
        content::{GetListCondition, NewContent},
        paginate::{Paginated, Pagination},
    },
};

#[async_trait::async_trait]
pub trait ContentDao {
    async fn create_content(
        &self,
        id: Uuid,
        owner_id: Uuid,
        new_content: &NewContent,
    ) -> InspirerContentResult<()>;
    async fn create_content_entity(
        &self,
        id: Uuid,
        new_content: &NewContent,
    ) -> InspirerContentResult<()>;
    async fn get_list(
        &self,
        condition: GetListCondition,
        pagination: Pagination,
    ) -> InspirerContentResult<Paginated<contents::Model>>;
    async fn find_content_by_id(
        &self,
        id: Uuid,
    ) -> InspirerContentResult<Option<(contents::Model, Option<content_entities::Model>)>>;
    async fn find_content_by_name(
        &self,
        name: String,
    ) -> InspirerContentResult<Option<(contents::Model, Option<content_entities::Model>)>>;
}

#[async_trait::async_trait]
impl<T: ConnectionTrait> ContentDao for T {
    async fn create_content(
        &self,
        id: Uuid,
        owner_id: Uuid,
        new_content: &NewContent,
    ) -> InspirerContentResult<()> {
        let model = contents::ActiveModel {
            id: Set(id),
            owner_id: Set(owner_id),
            title: Set(new_content.meta.title.clone()),
            keywords: Set(new_content.meta.keywords.clone()),
            description: Set(new_content.meta.description.clone()),
            content_name: Set(new_content.meta.name.clone()),
            content_type: Set(ContentType::from(&new_content.entity)),
            ..Default::default()
        };

        contents::Entity::insert(model).exec(self).await?;

        Ok(())
    }

    async fn create_content_entity(
        &self,
        id: Uuid,
        new_content: &NewContent,
    ) -> InspirerContentResult<()> {
        let model = content_entities::ActiveModel {
            id: Set(id),
            entity: Set(serde_json::to_value(&new_content.entity)
                .map_err(crate::error::Error::FormatError)?),
            ..Default::default()
        };

        content_entities::Entity::insert(model).exec(self).await?;

        Ok(())
    }

    async fn get_list(
        &self,
        condition: GetListCondition,
        pagination: Pagination,
    ) -> InspirerContentResult<Paginated<contents::Model>> {
        let mut selector = contents::Entity::find();

        if !condition.with_hidden {
            selector = selector.filter(contents::Column::IsDisplay.eq(true));
        }

        if !condition.with_unpublish {
            selector = selector.filter(contents::Column::IsPublish.eq(true));
        }

        if condition.without_page {
            selector = selector.filter(contents::Column::ContentType.ne(ContentType::Page));
        }

        let paginator = selector
            .order_by_desc(contents::Column::PublishedAt)
            .order_by_desc(contents::Column::CreatedAt)
            .paginate(self, pagination.page_size);

        let data = paginator.fetch_page(pagination.page - 1).await?;

        Ok(Paginated {
            data,
            page: pagination.page,
            page_size: pagination.page_size,
            total: paginator.num_items().await?,
            last_page: paginator.num_pages().await?,
        })
    }

    async fn find_content_by_id(
        &self,
        id: Uuid,
    ) -> InspirerContentResult<Option<(contents::Model, Option<content_entities::Model>)>> {
        contents::Entity::find_by_id(id)
            .find_also_related(content_entities::Entity)
            .one(self)
            .await
            .map_err(Into::into)
    }

    async fn find_content_by_name(
        &self,
        name: String,
    ) -> InspirerContentResult<Option<(contents::Model, Option<content_entities::Model>)>> {
        contents::Entity::find()
            .find_also_related(content_entities::Entity)
            .filter(contents::Column::ContentName.eq(Some(name)))
            .one(self)
            .await
            .map_err(Into::into)
    }
}
