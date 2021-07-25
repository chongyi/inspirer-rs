select id,
       creator_id,
       content_id,
       is_draft,
       title,
       keywords,
       description,
       created_at
from content_entities
where content_id = ?
  and is_draft = 1
order by created_at desc
limit 1