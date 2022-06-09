drop table if exists users;

alter table contents drop index index_content_owner;

alter table contents drop column owner_id;
alter table contents drop column authors;