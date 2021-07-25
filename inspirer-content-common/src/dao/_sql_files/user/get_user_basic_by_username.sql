select id, user_type, username, nickname, avatar, password
from users
where username = ?
limit 1