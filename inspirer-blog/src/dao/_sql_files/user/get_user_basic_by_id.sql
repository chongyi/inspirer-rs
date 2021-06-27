select id, user_type, username, nickname, password
from users
where id = ?
limit 1