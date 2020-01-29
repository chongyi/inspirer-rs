-- 内容表
create table if not exists contents
(
    id bigserial not null
        constraint contents_pk
            primary key,
    version varchar(64) not null,
    uuid char(32) not null,
    creator_uuid char(32) not null,
    title varchar,
    content_name varchar(255) default NULL::character varying,
    content_type smallint default 1 not null,
    keywords varchar(255) default ''::character varying not null,
    description varchar(500) default ''::character varying not null,
    display boolean default true not null,
    published boolean default false not null,
    published_at timestamp,
    created_at timestamp default CURRENT_TIMESTAMP not null,
    updated_at timestamp default CURRENT_TIMESTAMP not null
);

comment on table contents is '内容表';

comment on column contents.version is '版本 hash';
comment on column contents.creator_uuid is '创建人 UUID';
comment on column contents.uuid is '内容 UUID';
comment on column contents.title is '内容标题';
comment on column contents.content_name is '内容名称';
comment on column contents.content_type is '内容类型';
comment on column contents.published is '是否已经发布';
comment on column contents.published_at is '发布时间';

create unique index contents_content_name_uindex
    on contents (content_name);

create unique index contents_uuid_uindex
    on contents (uuid);

create index contents_published_index
    on contents (published);

create index contents_title_index
    on contents (title);

-- 内容实体表
create table content_entities
(
    content_uuid char(32) not null,
    version varchar(64) not null,
    content_body text,
    creator_uuid char(32) default null,
    constraint content_entities_pk
        primary key (content_uuid, version)
);

comment on table content_entities is '内容实体表';

comment on column content_entities.content_uuid is '内容 UUID';
comment on column content_entities.version is '版本 hash';
comment on column content_entities.creator_uuid is '内容贡献者';

create index content_entities_creator_index
    on content_entities (creator_uuid);

create index content_entities_content_uuid
    on content_entities (content_uuid);

-- 用户表
create table users
(
    id bigserial not null
        constraint users_pk
            primary key,
    uuid char(32) not null,
    invitor_uuid char(32) default null,
    password varchar(128) default null,
    user_type smallint default 1 not null,
    last_login timestamp default null,
    last_login_ip varchar(128) default null,
    login_count int default 0 not null,
    status smallint default 1 not null,
    activated_at timestamp default null,
    created_at timestamp default current_timestamp not null,
    updated_at timestamp default current_timestamp not null
);

comment on table users is '用户表';

comment on column users.uuid is '用户 UUID';
comment on column users.invitor_uuid is '邀请人 UUID';
comment on column users.password is '密码';
comment on column users.user_type is '用户类型，32767 为超级管理员（创始人）';
comment on column users.activated_at is '上次活跃的时间';

create unique index users_uuid_uindex
    on users (uuid);

-- 用户基础资料表
create table user_base_profiles
(
    user_uuid char(32) not null
        constraint user_base_profiles_pk
            primary key,
    nickname varchar(40) not null,
    avatar varchar(255) not null,
    gender smallint default 0 not null,
    created_at timestamp default current_timestamp not null,
    updated_at timestamp default current_timestamp not null
);

comment on table user_base_profiles is '用户基础资料表';

-- 用户邮箱凭据表
create table user_email_credentials
(
    email varchar(120) not null
        constraint user_email_credentials_pk
            primary key,
    user_uuid char(32) not null,
    status smallint default 0 not null,
    activated_at timestamp default null,
    created_at timestamp default current_timestamp not null,
    updated_at timestamp default current_timestamp not null
);

comment on table user_email_credentials is '用户邮箱凭据表';

comment on column user_email_credentials.activated_at is '邮箱激活时间';

create unique index user_email_credentials_user_uuid
    on user_email_credentials (user_uuid);

-- 用户手机号凭据表
create table user_mobile_phone_credentials
(
    country_code varchar(8) not null,
    mobile_phone varchar(32) not null,
    status smallint default 0 not null,
    created_at timestamp default current_timestamp not null,
    updated_at timestamp default current_timestamp not null,
    constraint user_mobile_phone_credentials_pk primary key (country_code, mobile_phone)
);

comment on table user_mobile_phone_credentials is '用户手机号凭据表';

comment on column user_mobile_phone_credentials.country_code is '国家编号';
comment on column user_mobile_phone_credentials.mobile_phone is '手机号码';

-- 验证代码表
create table validate_codes
(
    id bigserial not null
        constraint validate_codes_pk
            primary key,
    code varchar(16) not null,
    validate_target varchar not null,
    validate_channel smallint not null,
    is_validated bool default false not null,
    status bool default true not null,
    expired_at timestamp default null,
    created_at timestamp default current_timestamp not null,
    updated_at timestamp default current_timestamp not null
);

comment on table validate_codes is '验证代码表';

comment on column validate_codes.status is '有效性';
comment on column validate_codes.expired_at is '过期时间';

create unique index validate_codes_target_code_uindex
    on validate_codes (code, validate_target, validate_channel);

create index validate_codes_used_index
    on validate_codes (is_validated);

create index validate_codes_validate_index
    on validate_codes (validate_target asc, validate_channel asc, is_validated asc, created_at desc);