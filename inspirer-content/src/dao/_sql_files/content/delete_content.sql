update contents
set is_deleted = true,
    deleted_at = current_timestamp()
where id = ?