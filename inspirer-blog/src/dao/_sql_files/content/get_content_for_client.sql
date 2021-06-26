select contents.id,
       users.nickname creator_name,
       contents.creator_id,
       contents.title,
       contents.keywords,
       contents.description,
       contents.published_at,
       content_entities.content
from contents
         left join content_entities on contents.id = content_entities.content_id
         left join users on contents.creator_id = users.id
where contents.id = ?
  and is_deleted = false
order by content_entities.id desc
limit 1