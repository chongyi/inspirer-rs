use crate::model::content::*;
use crate::utils;
use chrono::prelude::*;
use crate::prelude::*;
use crate::schema::contents;
use crate::schema::users;
use crate::schema::content_entities;
use diesel::pg::expression::dsl::any;
use crate::model::user::BeJoinedUserBase;
use crate::model::content_entity::{ContentEntity, ContentEntityInsert};
use crate::utils::biz_err;

#[derive(Default, Deserialize)]
pub struct GetContentsIndex {
    pub per_page: Option<i64>,
    pub page: Option<i64>,
    pub content_type: Option<i16>,
    pub creator: Option<String>,
    pub title: Option<String>,
    pub published: Option<bool>,
    pub display: Option<bool>,
    pub owner: Option<String>,
}

impl ActiveModel for GetContentsIndex {
    type Result = ActionResult<PaginateWrapper<(ContentBase, BeJoinedUserBase)>>;

    fn activate(&self, conn: &PooledConn) -> Self::Result {
        let mut query = contents::table
            .left_join(users::table.on(users::user_uuid.eq(contents::creator_uuid)))
            .order((contents::published_at.desc(), contents::created_at.desc()))
            .into_boxed();

        if let Some(ref title) = self.title {
            query = query.filter(contents::title.ilike(format!("%{}%", title)));
        }

        if let Some(content_type) = self.content_type {
            query = query.filter(contents::content_type.eq(content_type));
        }

        if let Some(published) = self.published {
            query = query.filter(contents::published.eq(published));
        }

        if let Some(display) = self.display {
            query = query.filter(contents::display.eq(display));
        }

        if let Some(ref owner) = self.owner {
            query = query.filter(contents::creator_uuid.eq(owner));
        }

        if let Some(ref creator) = self.creator {
            let user_uuids = users::table
                .filter(
                    users::email.ilike(format!("%{}%", creator))
                        .or(users::mobile_phone.ilike(format!("%{}%", creator)))
                        .or(users::user_uuid.eq(creator))
                )
                .select(users::user_uuid)
                .load::<String>(conn)
                .map_err(ErrorKind::from)?;

            query = query.filter(contents::creator_uuid.eq(any(user_uuids)));
        }

        let mut query = query.select((
            content_base_columns,
            (
                users::id.nullable(),
                users::user_uuid.nullable(),
                users::nickname.nullable(),
                users::avatar.nullable(),
                users::status.nullable(),
                users::user_type.nullable()
            )
        ))
            .paginate(self.page.unwrap_or(1));

        if let Some(per_page) = self.per_page {
            query = query.per_page(per_page);
        }

        let (result, last_page, total) = query.load_and_count_pages::<(ContentBase, BeJoinedUserBase)>(conn).map_err(ErrorKind::from)?;

        Ok(PaginateWrapper {
            list: result,
            last_page,
            total,
        })
    }
}

pub enum GetContent<'i> {
    ByName(&'i str),
    ById(i64),
}

impl<'i> ActiveModel for GetContent<'i> {
    type Result = ActionResult<(ContentFull, ContentEntity, BeJoinedUserBase)>;

    fn activate(&self, conn: &PooledConn) -> Self::Result {
        let mut query = contents::table
            .left_join(users::table.on(users::user_uuid.eq(contents::creator_uuid)))
            .left_join(content_entities::table.on(content_entities::id.eq(contents::id).and(content_entities::version.eq(contents::version))))
            .into_boxed();

        match self {
            GetContent::ByName(ref name) => query = query.filter(contents::content_name.eq(name)),
            GetContent::ById(id) => query = query.filter(contents::id.eq(id)),
        };

        query.select((
            contents::all_columns,
            (
                content_entities::content_body.nullable(),
                content_entities::creator_uuid.nullable(),
            ),
            (
                users::id.nullable(),
                users::user_uuid.nullable(),
                users::nickname.nullable(),
                users::avatar.nullable(),
                users::status.nullable(),
                users::user_type.nullable()
            )
        ))
            .first::<(ContentFull, ContentEntity, BeJoinedUserBase)>(conn)
            .map_err(From::from)
    }
}

pub struct CreateContent<'i> {
    pub creator_uuid: &'i str,
    pub title: Option<&'i str>,
    pub content_name: Option<&'i str>,
    pub content_type: Option<i16>,
    pub keywords: Option<&'i str>,
    pub description: Option<&'i str>,
    pub display: Option<bool>,
    pub published: Option<bool>,
    pub published_at: Option<NaiveDateTime>,
    pub content: &'i str,
}

impl<'i> ActiveModel for CreateContent<'i> {
    type Result = QueryResult<i64>;

    fn activate(&self, conn: &PooledConn) -> Self::Result {
        let content = ContentInsert {
            version: 1,
            creator_uuid: self.creator_uuid,
            title: self.title,
            content_name: self.content_name,
            content_type: self.content_type.unwrap_or(1),
            keywords: self.keywords.unwrap_or(""),
            description: self.description.unwrap_or(""),
            display: self.display.unwrap_or(true),
            published: self.published.unwrap_or(false),
            published_at: self.published_at.or_else(|| {
                if let Some(published) = self.published {
                    if published {
                        return Some(Utc::now().naive_local());
                    }
                }
                None
            }),
        };

        let (id, version) = diesel::insert_into(contents::table)
            .values(&content)
            .returning((contents::id, contents::version))
            .get_result::<(i64, i32)>(conn)?;

        let content_entity = ContentEntityInsert {
            id,
            version,
            content_body: Some(self.content),
            creator_uuid: Some(self.creator_uuid),
        };

        diesel::insert_into(content_entities::table)
            .values(&content_entity)
            .execute(conn)?;

        Ok(id)
    }
}

pub enum PublishContent {
    Publish { id: i64, published_at: Option<NaiveDateTime> },
    Unpublish { id: i64 },
}

impl ActiveModel for PublishContent {
    type Result = QueryResult<usize>;

    fn activate(&self, conn: &PooledConn) -> Self::Result {
        let now = Utc::now().naive_local();
        let (target, published, published_at) = match self {
            PublishContent::Publish { id, published_at } => {
                let target = contents::table
                    .filter(contents::id.eq(id).and(contents::published.eq(false)));

                (target, true, Some(published_at.as_ref().unwrap_or(&now)))
            }
            PublishContent::Unpublish { id } => {
                let target = contents::table
                    .filter(contents::id.eq(id).and(contents::published.eq(true)));

                (target, false, None)
            }
        };

        diesel::update(contents::table)
            .set((
                contents::published.eq(published),
                contents::published_at.eq(published_at),
                contents::updated_at.eq(&now)
            ))
            .execute(conn)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use super::GetContentsIndex;
    use crate::model::user::BeJoinedUserBase;
    use crate::model::content::ContentBase;

    #[test]
    fn test_get_content_index() {
        auto_clear_base_environment(|conn| {
            let getter = GetContentsIndex {
                per_page: Some(2),
                page: Some(2),
                published: Some(true),
                display: Some(true),
                ..Default::default()
            };

            let result = getter.activate(conn).unwrap();
            println!("{:?}", result);
            let expect = PaginateWrapper {
                list: vec![
                    (ContentBase { id: 12, creator_uuid: "b9e87a68d0dd4748806e7ddb403701f5".to_string(), title: Some("ORANGE MARMALADE".to_string()), content_type: 2, display: true, published: true, created_at: utils::convert_to_native_datetime("2019-06-20 19:09:00").unwrap(), updated_at: utils::convert_to_native_datetime("2019-06-20 19:09:00").unwrap() }, BeJoinedUserBase { id: Some(1), user_uuid: Some("b9e87a68d0dd4748806e7ddb403701f5".to_string()), nickname: Some("administrator".to_string()), avatar: None, status: Some(1), member_type: Some(32767) }),
                    (ContentBase { id: 11, creator_uuid: "b9e87a68d0dd4748806e7ddb403701f5".to_string(), title: Some("Either the well was very deep".to_string()), content_type: 1, display: true, published: true, created_at: utils::convert_to_native_datetime("2019-06-20 19:08:00").unwrap(), updated_at: utils::convert_to_native_datetime("2019-06-20 19:08:00").unwrap() }, BeJoinedUserBase { id: Some(1), user_uuid: Some("b9e87a68d0dd4748806e7ddb403701f5".to_string()), nickname: Some("administrator".to_string()), avatar: None, status: Some(1), member_type: Some(32767) })
                ],
                last_page: 6,
                total: 12,
            };

            assert_eq!(expect, result);
        })
    }
}