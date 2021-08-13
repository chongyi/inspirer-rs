update content_entities
set content_id  = ?,
    is_draft    = ?,
    title       = ?,
    keywords    = ?,
    description = ?,
    content     = ?
where id = ?