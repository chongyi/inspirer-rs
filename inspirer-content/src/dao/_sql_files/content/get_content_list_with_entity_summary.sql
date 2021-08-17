select contents.id,
       contents.author_id,
       is_display,
       is_deleted,
       is_published,
       created_at,
       contents.updated_at,
       published_at,
       ce.title,
       ce.keywords,
       ce.description,
       count(*) over () total
from contents
         left join content_entities ce on contents.content_entity_id = ce.id

