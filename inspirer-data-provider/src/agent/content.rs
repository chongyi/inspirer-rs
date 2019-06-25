use crate::model::content::*;
use crate::utils;
use chrono::prelude::*;
use crate::prelude::*;
use crate::schema::contents;
use crate::schema::users;
use diesel::pg::expression::dsl::any;
use crate::model::user::BeJoinedUserBase;

#[derive(Default)]
pub struct GetContentsIndex<'i> {
    pub per_page: Option<i64>,
    pub page: Option<i64>,
    pub content_type: Option<i16>,
    pub creator: Option<&'i str>,
    pub title: Option<&'i str>,
    pub published: Option<bool>,
    pub display: Option<bool>,
    pub owner: Option<&'i str>,
}

impl<'i> ActiveModel for GetContentsIndex<'i> {
    type Result = QueryResult<PaginateWrapper<(ContentBase, BeJoinedUserBase)>>;

    fn activate(&self, conn: &PooledConn) -> Self::Result {
        let mut query = contents::table
            .left_join(users::table.on(users::user_uuid.eq(contents::creator_uuid)))
            .order((contents::published_at.desc(), contents::created_at.desc()))
            .into_boxed();

        if let Some(title) = self.title {
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

        if let Some(owner) = self.owner {
            query = query.filter(contents::creator_uuid.eq(owner));
        }

        if let Some(creator) = self.creator {
            let user_uuids = users::table
                .filter(
                    users::email.ilike(format!("%{}%", creator))
                        .or(users::mobile_phone.ilike(format!("%{}%", creator)))
                        .or(users::user_uuid.eq(creator))
                )
                .select(users::user_uuid)
                .load::<String>(conn)?;

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

        let (result, last_page, total) = query.load_and_count_pages::<(ContentBase, BeJoinedUserBase)>(conn)?;

        Ok(PaginateWrapper {
            data: result,
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
    type Result = QueryResult<(ContentFull, BeJoinedUserBase)>;

    fn activate(&self, conn: &PooledConn) -> Self::Result {
        let mut query = contents::table
            .left_join(users::table.on(users::user_uuid.eq(contents::creator_uuid)))
            .into_boxed();

        match self {
            GetContent::ByName(ref name) => query = query.filter(contents::content_name.eq(name)),
            GetContent::ById(id) => query = query.filter(contents::id.eq(id)),
        };

        query.select((
                contents::all_columns,
                (
                    users::id.nullable(),
                    users::user_uuid.nullable(),
                    users::nickname.nullable(),
                    users::avatar.nullable(),
                    users::status.nullable(),
                    users::user_type.nullable()
                )
            ))
            .first::<(ContentFull, BeJoinedUserBase)>(conn)
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
                data: vec![
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