-- 内容表
create table if not exists contents
(
    id           bigserial                                  not null
        constraint contents_pk
            primary key,
    version      int          default 0                     not null,
    creator_uuid char(32)                                   not null,
    title        varchar,
    content_name varchar(255) default NULL::character varying,
    content_type smallint     default 1                     not null,
    keywords     varchar(255) default ''::character varying not null,
    description  varchar(500) default ''::character varying not null,
    display      boolean      default true                  not null,
    published    boolean      default false                 not null,
    published_at timestamp,
    created_at   timestamp    default CURRENT_TIMESTAMP     not null,
    updated_at   timestamp    default CURRENT_TIMESTAMP     not null
);

comment on table contents is '内容表';

comment on column contents.version is '版本序号';
comment on column contents.creator_uuid is '创建人 UUID';
comment on column contents.title is '内容标题';
comment on column contents.content_name is '内容名称';
comment on column contents.content_type is '内容类型';
comment on column contents.published is '是否已经发布';
comment on column contents.published_at is '发布时间';

create unique index contents_content_name_uindex
    on contents (content_name);

create index contents_published_index
    on contents (published);

create index contents_title_index
    on contents (title);

-- 内容实体表
create table content_entities
(
    id           bigint             not null,
    version      int      default 1 not null,
    content_body text,
    creator_uuid char(32) default null,
    constraint content_entities_pk
        primary key (id, version)
);

comment on table content_entities is '内容实体表';

comment on column content_entities.id is '内容 ID';
comment on column content_entities.version is '版本序号';
comment on column content_entities.creator_uuid is '内容贡献者';

create index content_entities_creator_index
    on content_entities (creator_uuid);

-- 用户表
create table users
(
    id            bigserial                              not null
        constraint users_pk
            primary key,
    user_uuid     char(32)                               not null,
    invitor_uuid  char(32)     default null,
    email         varchar(140) default null,
    mobile_phone  varchar(40)  default null,
    country_code  varchar(12)  default null,
    password      varchar(128) default null,
    nickname      varchar(60)                            not null,
    gender        bool         default null,
    avatar        varchar(500) default null,
    user_type     smallint     default 1                 not null,
    last_login    timestamp    default null,
    last_login_ip varchar(128) default null,
    login_count   int          default 0                 not null,
    status        smallint     default 1                 not null,
    activated_at  timestamp    default null,
    created_at    timestamp    default current_timestamp not null,
    updated_at    timestamp    default current_timestamp not null
);

comment on table users is '用户表';

comment on column users.invitor_uuid is '邀请人 UUID';
comment on column users.email is '邮箱';
comment on column users.mobile_phone is '手机号';
comment on column users.country_code is '国家代码';
comment on column users.password is '密码';
comment on column users.gender is '性别';
comment on column users.avatar is '头像';
comment on column users.user_type is '用户类型，32767 为超级管理员（创始人）';
comment on column users.activated_at is '上次活跃的时间';

create unique index users_email_uindex
    on users (email);

create unique index users_mobile_phone_uindex
    on users (country_code, mobile_phone);

create unique index users_user_uuid_uindex
    on users (user_uuid);

-- 验证代码表
create table validate_codes
(
    id               bigserial                           not null
        constraint validate_codes_pk
            primary key,
    code             varchar(16)                         not null,
    validate_target  varchar                             not null,
    validate_channel smallint                            not null,
    is_validated     bool      default false             not null,
    status           bool      default true              not null,
    expired_at       timestamp default null,
    created_at       timestamp default current_timestamp not null,
    updated_at       timestamp default current_timestamp not null
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