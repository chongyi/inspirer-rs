-- auto-generated definition
create table contents
(
    id                bigint unsigned auto_increment
        primary key,
    creator_id        bigint unsigned default '0'               not null,
    title             varchar(140)    default ''                not null,
    keywords          varchar(255)    default ''                not null,
    description       varchar(255)    default ''                not null,
    content_entity_id bigint unsigned                           not null comment '内容实体 ID',
    is_display        tinyint(1)      default 0                 not null comment '是否可见，即表示不直接展示，但能够访问到的',
    is_published      tinyint(1)      default 0                 not null comment '是否发布，不同于可见性，未发布内容无法通过任何手段看见',
    published_at      timestamp       default CURRENT_TIMESTAMP not null comment '发布时间',
    created_at        timestamp       default CURRENT_TIMESTAMP not null,
    updated_at        timestamp       default CURRENT_TIMESTAMP not null on update CURRENT_TIMESTAMP,
    constraint contents_content_entity_id_uindex
        unique (content_entity_id)
)
    comment '内容表';

create index contents_creator_id_index
    on contents (creator_id);


-- auto-generated definition
create table content_entities
(
    id          bigint unsigned auto_increment
        primary key,
    creator_id  bigint unsigned default '0'               not null,
    content_id  bigint unsigned default '0'               not null,
    is_draft    tinyint(1)                                not null comment '是否是草稿',
    title       varchar(140)    default ''                not null,
    keywords    varchar(255)    default ''                not null,
    description varchar(255)    default ''                not null,
    content     mediumtext                                not null,
    created_at  timestamp       default CURRENT_TIMESTAMP not null
)
    comment '内容实体表';

create index content_entities_content_id_index
    on content_entities (content_id);

create index content_entities_creator_id_index
    on content_entities (creator_id);

