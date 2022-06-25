use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};
use uuid::Uuid;

use crate::{
    entity::content_entities,
    entity::{contents, content_update_logs},
    enumerate::content::ContentType,
    error::{Error, InspirerContentResult},
    model::{
        content::{GetListCondition, NewContent, UpdateContent},
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
    async fn update_content(
        &self,
        id: Uuid,
        update_content: &UpdateContent,
    ) -> InspirerContentResult<()>;
    async fn update_content_entity(
        &self,
        id: Uuid,
        update_content: &UpdateContent,
    ) -> InspirerContentResult<()>;
    async fn delete_content(&self, id: Uuid) -> InspirerContentResult<()>;
    async fn delete_content_entity(&self, id: Uuid) -> InspirerContentResult<()>;
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

    async fn update_content(
        &self,
        id: Uuid,
        update_content: &UpdateContent,
    ) -> InspirerContentResult<()> {
        if update_content.meta.title.is_none()
            && update_content.meta.description.is_none()
            && update_content.meta.keywords.is_none()
            && update_content.meta.name.is_none()
            && update_content.entity.is_none()
        {
            return Ok(());
        }

        let model = contents::Entity::find_by_id(id)
            .one(self)
            .await?
            .ok_or(Error::ContentNotFound)?;

        let mut active_model: contents::ActiveModel = model.into();

        if let Some(title) = &update_content.meta.title {
            active_model.title = Set(title.clone());
        }

        if let Some(description) = &update_content.meta.description {
            active_model.description = Set(description.clone());
        }

        if let Some(keywords) = &update_content.meta.keywords {
            active_model.keywords = Set(keywords.clone());
        }

        if let Some(name) = &update_content.meta.name {
            active_model.content_name = Set(Some(name.clone()));
        }

        if let Some(entity) = &update_content.entity {
            active_model.content_type = Set(entity.into());
        }

        active_model.update(self).await?;

        Ok(())
    }

    async fn update_content_entity(
        &self,
        id: Uuid,
        update_content: &UpdateContent,
    ) -> InspirerContentResult<()> {
        if update_content.entity.is_none() {
            return Ok(());
        }

        let model = content_entities::Entity::find_by_id(id)
            .one(self)
            .await?
            .ok_or(Error::ContentNotFound)?;
        let mut active_model: content_entities::ActiveModel = model.into();

        if let Some(entity) = &update_content.entity {
            active_model.entity =
                Set(serde_json::to_value(entity).map_err(crate::error::Error::FormatError)?);
        }

        active_model.update(self).await?;

        Ok(())
    }

    async fn delete_content(&self, id: Uuid) -> InspirerContentResult<()> {
        contents::Entity::delete_by_id(id).exec(self).await?;

        Ok(())
    }

    async fn delete_content_entity(&self, id: Uuid) -> InspirerContentResult<()> {
        content_entities::Entity::delete_by_id(id)
            .exec(self)
            .await?;

        Ok(())
    }
}

#[async_trait::async_trait]
pub trait ContentUpdateLogDao {
    async fn create_content_update_log(
        &self,
        id: Uuid,
        user_id: Uuid,
        content_id: Uuid,
        update_content: UpdateContent,
    ) -> InspirerContentResult<()>;
}

#[async_trait::async_trait]
impl<T: ConnectionTrait> ContentUpdateLogDao for T {
    async fn create_content_update_log(
        &self,
        id: Uuid,
        user_id: Uuid,
        content_id: Uuid,
        update_content: UpdateContent,
    ) -> InspirerContentResult<()> {
        let model = content_update_logs::ActiveModel {
            id: Set(id),
            user_id: Set(user_id),
            content_id: Set(content_id),
            update_data: Set(serde_json::to_value(update_content).map_err(crate::error::Error::FormatError)?),
            ..Default::default()
        };

        content_update_logs::Entity::insert(model).exec(self).await?;

        Ok(())
    }
}
