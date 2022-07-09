alter table contents add column is_deleted boolean not null default false after is_display;
alter table contents add column deleted_at timestamp null after published_at;
alter table contents add column modified_at timestamp not null default current_timestamp after updated_at;

update contents set modified_at = updated_at;