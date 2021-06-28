select id, user_type, username, nickname, avatar, password
from users
where id = ?
limit 1