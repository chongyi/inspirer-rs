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
  `title` varchar(255) character set utf8mb4 collate utf8mb4_general_ci not null,
  `category_id` int unsigned default null,
  `keywords` varchar(255) character set utf8mb4 collate utf8mb4_general_ci not null default '',
  `description` varchar(500) character set utf8mb4 collate utf8mb4_general_ci not null default '',
  `sort` smallint not null default 0,
  `type` smallint unsigned not null default 1,
  `content` mediumtext character set utf8mb4 collate utf8mb4_general_ci null,
  `display` tinyint(1) unsigned not null default 1,
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
  `sort` smallint not null default 0,
  `created_at` timestamp not null default current_timestamp,
  `updated_at` timestamp null on update current_timestamp,
  primary key (`id`),
  key `search` (`content`)
) character set = utf8mb4 collate = utf8mb4_general_ci comment = 'PUSH 消息表';

create table if not exists `subjects` (
  `id` int unsigned not null auto_increment,
  `name` varchar(255) character set utf8mb4 collate utf8mb4_general_ci,
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