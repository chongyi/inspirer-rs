create table if not exists contents (
    id binary(16) not null primary key,
    content_name varchar(240) default null,
    content_type int unsigned not null,
    title varchar(240) not null,
    keywords varchar(500) not null,
    description varchar(500) not null,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp on update current_timestamp,
    is_publish boolean not null default false,
    is_display boolean not null default true,
    published_at timestamp null
);

create unique index unique_content_name on contents (content_name);

create table if not exists content_entities (
    id binary(16) not null primary key,
    entity json not null,
    updated_at timestamp not null default current_timestamp on update current_timestamp
);