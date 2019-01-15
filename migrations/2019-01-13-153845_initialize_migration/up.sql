create table if not exists `categories` (
  `id` int unsigned not null auto_increment,
  `name` varchar(255) character set utf8mb4 collate utf8mb4_general_ci not null,
  `display_name` varchar(255) character set utf8mb4 collate utf8mb4_general_ci not null,
  `keywords` varchar(255) character set utf8mb4 collate utf8mb4_general_ci not null default '',
  `description` varchar(500) character set utf8mb4 collate utf8mb4_general_ci not null default '',
  `sort` smallint(4) not null default 0,
  `created_at` timestamp not null default current_timestamp,
  `updated_at` timestamp null on update current_timestamp,
  primary key (`id`),
  unique key `union_name` (`name`)
) character set = utf8mb4 collate = utf8mb4_general_ci comment = '内容分类表';

create table if not exists `contents` (
  `id` int unsigned not null auto_increment,
  `name` varchar(255) character set utf8mb4 collate utf8mb4_general_ci,
  `cover` varchar(500) default null comment '内容封面',
  `title` varchar(255) character set utf8mb4 collate utf8mb4_general_ci not null,
  `category_id` int unsigned default null,
  `as_page` tinyint(1) not null default 0 comment '是否可以作为单页面',
  `allow_comment` tinyint(1) not null default 1 comment '是否允许评论',
  `limit_comment` tinyint not null default 2 comment '限制评论，当 `allow_comment` 为 1 时该值有意义。 1 表不限制 2 表需审核',
  `keywords` varchar(255) character set utf8mb4 collate utf8mb4_general_ci not null default '',
  `description` varchar(500) character set utf8mb4 collate utf8mb4_general_ci not null default '',
  `sort` smallint not null default 0,
  `content_type` smallint unsigned not null default 1,
  `content` mediumtext character set utf8mb4 collate utf8mb4_general_ci null,
  `display` tinyint(1) not null default 1,
  `published_at` timestamp null,
  `modified_at` timestamp null,
  `created_at` timestamp not null default current_timestamp,
  `updated_at` timestamp null on update current_timestamp,
  primary key (`id`),
  unique key `union_name` (`name`),
  key `search_title` (`name`, `title`)
) character set = utf8mb4 collate = utf8mb4_general_ci comment = '内容表';

create table if not exists `push_messages` (
  `id` int unsigned not null auto_increment,
  `content` varchar(500) character set utf8mb4 collate utf8mb4_general_ci not null,
  `allow_comment` tinyint(1) not null default 1 comment '是否允许评论',
  `limit_comment` tinyint not null default 1 comment '限制评论，当 `allow_comment` 为 1 时该值有意义。 1 表不限制 2 表需审核',
  `sort` smallint not null default 0,
  `created_at` timestamp not null default current_timestamp,
  `updated_at` timestamp null on update current_timestamp,
  primary key (`id`),
  key `search` (`content`)
) character set = utf8mb4 collate = utf8mb4_general_ci comment = 'PUSH 消息表';

create table if not exists `subjects` (
  `id` int unsigned not null auto_increment,
  `name` varchar(255) character set utf8mb4 collate utf8mb4_general_ci,
  `cover` varchar(500) default null comment '专题封面',
  `title` varchar(255) character set utf8mb4 collate utf8mb4_general_ci not null,
  `keywords` varchar(255) character set utf8mb4 collate utf8mb4_general_ci not null default '',
  `description` varchar(500) character set utf8mb4 collate utf8mb4_general_ci not null default '',
  `sort` smallint not null default 0,
  `created_at` timestamp not null default current_timestamp,
  `updated_at` timestamp null on update current_timestamp,
  primary key (`id`),
  unique key `union_name` (`name`),
  key `search_title` (`name`, `title`)
) character set = utf8mb4 collate = utf8mb4_general_ci comment = '专题表';

create table if not exists `subject_relates` (
  `subject_id` int unsigned not null,
  `content_id` int unsigned not null,
  `sort` smallint not null default 0,
  `created_at` timestamp not null default current_timestamp,
  `updated_at` timestamp null on update current_timestamp,
  primary key (`subject_id`, `content_id`)
) comment = '专题内容关联表';

create table if not exists `recommend_contents` (
  `id` int unsigned not null auto_increment,
  `content_id` int unsigned default null,
  `source` varchar(500) character set utf8mb4 collate utf8mb4_general_ci not null,
  `title` varchar(255) character set utf8mb4 collate utf8mb4_general_ci not null,
  `summary` varchar(500) character set utf8mb4 collate utf8mb4_general_ci not null,
  `created_at` timestamp not null default current_timestamp,
  `updated_at` timestamp null on update current_timestamp,
  primary key (`id`),
  key `content` (`content_id`)
) character set = utf8mb4 collate = utf8mb4_general_ci comment = '推荐内容表';

create table if not exists `links` (
  `id` int unsigned not null auto_increment,
  `title` varchar(255) character set utf8mb4 collate utf8mb4_general_ci not null,
  `link` varchar(500) not null,
  `sort` smallint unsigned not null default 0,
  `icon` varchar(500) default null,
  `created_at` timestamp not null default current_timestamp ,
  `updated_at` timestamp null on update current_timestamp ,
  primary key (`id`)
) character set = utf8mb4 collate = utf8mb4_general_ci comment = '友情链接表';

create table if not exists `members` (
  `id` int unsigned not null auto_increment,
  `nickname` varchar(40) not null,
  `avatar` varchar(500) default null comment '头像',
  `gender` tinyint not null default 0 comment '性别，0 未知 1 男 2 女',
  `email` varchar(80) not null,
  `email_is_valid` tinyint(1) not null default 0 comment '邮箱是否有效',
  `email_verified_at` timestamp null comment '邮箱验证时间',
  `password` varchar(255) default null,
  `status` tinyint not null default 0 comment '状态，0 表无效 1 表有效',
  `activated_at` timestamp null comment '上次活跃时间',
  `created_at` timestamp not null default current_timestamp ,
  `updated_at` timestamp null on update current_timestamp ,
  primary key (`id`),
  unique key email_account (`email`)
) character set = utf8mb4 collate = utf8mb4_general_ci comment = '会员表';