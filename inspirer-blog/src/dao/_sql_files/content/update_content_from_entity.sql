update contents, (select id, title, keywords, description
                  from content_entities
                  where id = ? and is_draft = ?
                  order by created_at desc) as content_entities
set contents.title       = content_entities.title,
    contents.keywords    = content_entities.keywords,
    contents.description = content_entities.description
where contents.id = content_entities.id