use chrono::NaiveDateTime;
use diesel;
use diesel::*;

use super::GetDescription;
use schema::content_articles;

pub struct ArticleDisplay {
    pub id: u32,
    pub content: String,
    pub name: Option<String>,
    pub views: u32,
    pub modified_at: Option<NaiveDateTime>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CreateArticle {
    pub content: String,
    pub name: Option<String>,
}

impl GetDescription for CreateArticle {
    fn description(&self) -> String {
        let sub = self.content.chars().into_iter().map(|x| x.to_string()).collect::<Vec<_>>();
        let len = sub.len();
        if len > 240 {
            sub[..240].concat()
        } else {
            sub[..len].concat()
        }
    }
}

#[derive(Insertable)]
#[table_name = "content_articles"]
pub struct NewArticle {
    pub content: String,
    pub name: Option<String>,
}

impl From<CreateArticle> for NewArticle {
    fn from(create: CreateArticle) -> Self {
        NewArticle {
            content: create.content.clone(),
            name: create.name.clone(),
        }
    }
}