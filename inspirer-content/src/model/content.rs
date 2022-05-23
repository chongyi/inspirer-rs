pub struct NewContent {
    title: String,
    keywords: Option<String>,
    description: Option<String>,
    name: Option<String>,
    entity: ContentEntity,
}

pub enum ContentEntity {
    Post(PostContent)
}

pub struct PostContent {
    content: String,
}