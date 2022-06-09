create table if not exists users (
    id binary(16) not null primary key,
    username varchar(40) not null,
    nickname varchar(40) not null,
    avatar varchar(500) not null default '',
    user_profile json default (json_object()),
    public_key varbinary(500) not null,
    public_key_fingerprint binary(32) not null,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp on update current_timestamp
);

create unique index unique_username on users (username);
create unique index unique_pubkey on users (public_key_fingerprint);

alter table contents add column owner_id binary(16) not null default 0 after id;
alter table contents add column authors json default (json_array()) after owner_id;

create index index_content_owner on contents (owner_id);