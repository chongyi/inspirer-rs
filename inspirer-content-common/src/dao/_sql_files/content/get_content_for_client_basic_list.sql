select contents.id,
       users.nickname   creator_name,
       contents.creator_id,
       contents.title,
       contents.keywords,
       contents.description,
       contents.published_at,
       count(*) over () total
from contents
         left join users on contents.creator_id = users.id