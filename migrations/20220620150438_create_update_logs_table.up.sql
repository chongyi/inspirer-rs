create table if not exists content_update_logs (
    id binary(16) not null primary key,
    user_id binary(16) not null,
    content_id binary(16) not null,
    update_data json not null,
    app_version int unsigned not null default 1,
    created_at timestamp not null default current_timestamp
);

create index index_user on content_update_logs (user_id);

create index index_content on content_update_logs (content_id);

insert into
    content_update_logs (id, user_id, content_id, update_data, created_at)
select
    uuid_to_bin(uuid()),
    owner_id,
    contents.id,
    json_object(
        "title",
        title,
        "description",
        description,
        "keywords",
        keywords,
        "content_name",
        content_name,
        "content_type",
        content_type,
        "entity",
        content_entities.entity
    ),
    contents.created_at
from
    contents
    left join content_entities on content_entities.id = contents.id;