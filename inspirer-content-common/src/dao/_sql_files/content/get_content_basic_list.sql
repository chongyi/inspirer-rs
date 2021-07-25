select contents.id,
       contents.creator_id,
       users.nickname creator_name,
       contents.title,
       contents.keywords,
       contents.description,
       contents.is_display,
       contents.is_published,
       contents.published_at,
       contents.created_at,
       contents.updated_at,
       count(*) over () total
from contents
         left join users on users.id = contents.creator_id