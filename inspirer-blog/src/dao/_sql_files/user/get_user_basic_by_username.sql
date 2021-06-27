select id, user_type, username, nickname, password
from users
where username = ?
limit 1