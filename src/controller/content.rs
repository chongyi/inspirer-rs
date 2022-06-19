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
    request::content::CreateContent,
    response::{
        content::{ContentBase, ContentFull, ContentFullWithEntity, ContentWithEntity},
        CreatedDataStringId,
    },
    session::SessionInfo,
};

pub async fn get_content_list_simple(
    Query(pagination): Query<Pagination>,
    Extension(manager): Extension<Manager>,
) -> InspirerResult<Json<Paginated<ContentBase>>> {
    manager
        .get_list(
            GetListCondition {
                with_hidden: false,
                with_unpublish: false,
                without_page: true,
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

pub async fn create_content(
    Extension(manager): Extension<Manager>,
    session: SessionInfo,
    Json(payload): Json<CreateContent>,
) -> InspirerResult<Json<CreatedDataStringId>> {
    manager
        .create_content(session.uuid(), payload)
        .await
        .map_err(Into::into)
        .map(CreatedDataStringId::from_uuid)
        .map(Json)
}

/// 获取内容列表（需要授权登录）
pub async fn get_content_list(
    Extension(manager): Extension<Manager>,
    // session: SessionInfo,
    Query(pagination): Query<Pagination>,
) -> InspirerResult<Json<Paginated<ContentFull>>> {
    manager
        .get_list(
            GetListCondition {
                with_hidden: true,
                with_unpublish: true,
                without_page: false,
            },
            pagination,
        )
        .await
        .map(|res| res.map(|data| data.into_iter().map(ContentFull::from).collect()))
        .map_err(Into::into)
        .map(Json)
}

pub async fn get_content(
    Extension(manager): Extension<Manager>,
    Path((id,)): Path<(String,)>,
    session: SessionInfo,
) -> InspirerResult<Json<ContentFullWithEntity>> {
    manager
        .find_content_by_id(base62_to_uuid(&id)?)
        .await
        .map(|content| ContentFullWithEntity {
            content: ContentFull::from(content.meta),
            entity: content.entity,
        })
        .map(Json)
        .map_err(Into::into)
}
