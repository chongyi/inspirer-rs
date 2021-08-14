select *
from content_entities
where content_id = ?
  and is_draft = ?
order by updated_at desc, id desc
limit 1