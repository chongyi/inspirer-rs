use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use inspirer_content::{
    error::Error,
    manager::Manager,
    model::{
        content::{Content, GetListCondition},
        paginate::{Paginated, Pagination},
    },
    service::content::ContentService,
    util::uuid::base62_to_uuid,
};

use crate::{
    error::InspirerResult,
    response::content::{ContentBase, ContentWithEntity},
};

pub async fn get_content_list(
    Query(pagination): Query<Pagination>,
    Extension(manager): Extension<Manager>,
) -> InspirerResult<Json<Paginated<ContentBase>>> {
    manager
        .get_list(
            GetListCondition {
                with_hidden: false,
                with_unpublish: false,
            },
            pagination,
        )
        .await
        .map(|res| res.map(|data| data.into_iter().map(ContentBase::from).collect()))
        .map_err(Into::into)
        .map(Json)
}

pub async fn find_content(
    Path((id,)): Path<(String,)>,
    Extension(manager): Extension<Manager>,
) -> InspirerResult<Json<ContentWithEntity>> {
    let Content {
        meta: content_raw,
        entity,
    } = match manager.find_content_by_name(id.clone()).await {
        Ok(res) => res,
        Err(Error::ContentNotFound) => manager.find_content_by_id(base62_to_uuid(&id)?).await?,
        Err(err) => Err(err)?,
    };

    Ok(Json(ContentWithEntity {
        base: ContentBase::from(content_raw),
        entity,
    }))
}
