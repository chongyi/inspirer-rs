create table if not exists `categories` (
  `id` int unsigned not null auto_increment,
  `parent_id` int unsigned not null default 0 comment '父分类 ID',
  `path` varchar(500) not null default '0' comment '分类路径',
  `name` varchar(255) character set utf8mb4 collate utf8mb4_general_ci not null,
  `display_name` varchar(255) character set utf8mb4 collate utf8mb4_general_ci not null,
  `keywords` varchar(255) character set utf8mb4 collate utf8mb4_general_ci not null default '',
  `description` varchar(500) character set utf8mb4 collate utf8mb4_general_ci not null default '',
  `sort` smallint(4) not null default 0,
  `created_at` timestamp not null default current_timestamp,
  `updated_at` timestamp null on update current_timestamp,
  primary key (`id`),
  unique key `union_name` (`name`),
  key `index_parent` (`parent_id`),
  key `index_path` (`path`),
  key `index_display_name` (`display_name`)
) character set = utf8mb4 collate = utf8mb4_general_ci comment = '内容分类表';

create table if not exists `contents` (
  `id` int unsigned not null auto_increment,
  `creator_id` int unsigned not null comment '创建人 ID（会员 ID）',
  `creator_nickname` varchar(40) not null comment '创建人昵称',
  `creator_avatar` varchar(500) default null comment '头像',
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
  key `index_title` (`title`),
  key `index_name` (`name`),
  key `index_search` (`name`, `title`),
  key `index_creator` (`creator_id`)
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
  `summary` varchar(500) character set utf8mb4 collate utf8mb4_general_ci not null comment '内容摘要',
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
  `mobile_phone` varchar(30) default null comment '手机号码',
  `country_code` varchar(8) not null default '86' comment '国家代码，默认 86',
  `email` varchar(80) not null,
  `email_is_valid` tinyint(1) not null default 0 comment '邮箱是否有效',
  `email_verified_at` timestamp null comment '邮箱验证时间',
  `password` varchar(255) default null,
  `status` tinyint not null default 0 comment '状态，0 表无效 1 表有效',
  `activated_at` timestamp null comment '上次活跃时间',
  `created_at` timestamp not null default current_timestamp ,
  `updated_at` timestamp null on update current_timestamp ,
  primary key (`id`),
  unique key email_account (`email`),
  key index_nickname (`nickname`),
  key index_gender (`gender`),
  key index_status (`status`)
) character set = utf8mb4 collate = utf8mb4_general_ci comment = '会员表';

create table if not exists `platform_accounts` (
  `id` int unsigned not null auto_increment,
  `account` varchar(40) not null,
  `password` varchar(255) default null,
  `is_system` tinyint(1) not null default 0 comment '是否为系统账户',
  `email` varchar(80) not null,
  `email_is_valid` tinyint(1) not null default 0 comment '邮箱是否有效',
  `email_verified_at` timestamp null comment '邮箱验证时间',
  `member_id` int unsigned default null comment '绑定的会员 ID',
  `member_bound_at` timestamp null comment '绑定会员的时间',
  `role_id` int unsigned not null default 0 comment '角色 ID',
  `status` tinyint(1) not null default 1 comment '可用状态',
  `activated_at` timestamp null comment '上次活跃时间',
  `created_at` timestamp not null default current_timestamp ,
  `updated_at` timestamp null on update current_timestamp ,
  primary key (`id`),
  unique key unique_accounr (`account`),
  unique key unique_bound (`account`, `member_id`)
) character set = utf8mb4 collate = utf8mb4_general_ci comment = '平台账户表';

create table if not exists `platform_roles` (
  `id` int unsigned not null auto_increment,
  `name` varchar(60) not null comment '角色标识符',
  `display_name` varchar(80) not null comment '角色名称',
  `description` varchar(500) not null default '' comment '角色描述',
  `icon` varchar(500) default null comment '角色图标',
  `is_system` tinyint(1) not null default 0 comment '是否为系统角色',
  `is_super` tinyint(1) not null default 0 comment '是否为最高权限角色（忽略授权表）',
  `status` tinyint(1) not null default 1 comment '可用状态',
  `created_at` timestamp not null default current_timestamp ,
  `updated_at` timestamp null on update current_timestamp ,
  primary key (`id`),
  unique key unique_name (`name`),
  key index_display_name (`display_name`),
  key index_description (`description`)
) character set = utf8mb4 collate = utf8mb4_general_ci comment = '平台角色表';

create table if not exists `platform_role_relates` (
  `account_id` int unsigned not null,
  `role_id` int unsigned not null,
  `created_at` timestamp not null default current_timestamp,
  `updated_at` timestamp null on update current_timestamp,
  primary key (`account_id`, `role_id`),
  key index_account (`account_id`),
  key index_role (`role_id`)
) character set = utf8mb4 collate = utf8mb4_general_ci comment = '平台角色关联表';

create table if not exists `platform_permissions` (
  `id` int unsigned not null auto_increment,
  `name` varchar(60) not null comment '权限标识符',
  `display_name` varchar(80) not null comment '权限名称',
  `description` varchar(500) not null default '' comment '权限描述',
  `model` varchar(120) not null comment '权限模型',
  `model_parameters` text comment '默认权限模型参数',
  `is_system` tinyint(1) not null default 0 comment '是否为系统权限',
  `created_at` timestamp not null default current_timestamp ,
  `updated_at` timestamp null on update current_timestamp ,
  primary key (`id`),
  unique key unique_name (`name`),
  key index_display_name (`display_name`),
  key index_description (`description`)
) character set = utf8mb4 collate = utf8mb4_general_ci comment = '平台权限表';

create table if not exists `platform_permission_relates` (
  `target_id` int unsigned not null,
  `target_type` tinyint not null comment '关联对象类型，1 表角色 2 表平台账户',
  `permission_id` int unsigned not null,
  `status` tinyint(1) not null default 1 comment '启用状态',
  `created_at` timestamp not null default current_timestamp,
  `updated_at` timestamp null on update current_timestamp,
  primary key (`target_id`, `target_type`, `permission_id`),
  key index_target (`target_id`, `target_type`),
  key index_permission (`permission_id`)
) character set = utf8mb4 collate = utf8mb4_general_ci comment = '平台权限关联表';