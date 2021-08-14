select contents.id,
       contents.author_id,
       contents.content_entity_id,
       contents.is_display,
       contents.is_published,
       contents.is_deleted,
       contents.published_at,
       contents.created_at,
       contents.updated_at,
       contents.deleted_at,
       ce.is_draft,
       ce.title,
       ce.keywords,
       ce.description,
       ce.content
from contents
         left join content_entities ce on ce.id = contents.content_entity_id
where contents.id = ?
  and is_deleted = false
limit 1