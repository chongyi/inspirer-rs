create table if not exists `discussants` (
  `id` int unsigned not null auto_increment,
  `nickname` varchar(40) not null,
  `email` varchar(80) not null,
  `password` varchar(255) default null,
  `status` tinyint not null default 0,
  `activated_at` timestamp null,
  `created_at` timestamp not null default current_timestamp ,
  `updated_at` timestamp null on update current_timestamp ,
  primary key (`id`),
  unique key email_account (`email`)
) character set = utf8mb4 collate = utf8mb4_general_ci comment = '讨论者表';

create table if not exists `comments` (
  `id` int unsigned not null auto_increment,
  `channel` smallint unsigned not null comment '讨论主题频道',
  `discussant_id` int unsigned not null,
  `subject_id` int unsigned not null comment '讨论主题 ID',
  `discussion_id` int unsigned not null default 0 comment '讨论 ID，即评论主 ID',
  `reply_id` int unsigned not null default 0 comment '回复对象 ID',
  `reply_discussant_id` int unsigned default null,
  `replyable` tinyint(1) not null default 1,
  `content` text character set utf8mb4 collate utf8mb4_general_ci not null,
  `display` tinyint(1) not null default 1,
  `verified` tinyint(1) not null default 0,
  `verifier_type` tinyint not null default 1 comment '审核人类型，1 表自动，2 表人工',
  `verified_at` timestamp not null comment '审核通过时间',
  `created_at` timestamp not null default current_timestamp ,
  `updated_at` timestamp null on update current_timestamp ,
  primary key (`id`),
  key `subject_index` (`channel`, `subject_id`),
  key `display_index` (`display`),
  key `discussion_index` (`discussion_id`),
  key `owner_index` (`discussant_id`)
) character set = utf8mb4 collate = utf8mb4_general_ci comment = '内容评论表';

alter table `contents` add column `allow_comment` tinyint(1) not null default 1 comment '是否允许评论' after `as_page`;
alter table `contents` add column `limit_comment` tinyint not null default 2 comment '限制评论，当 `allow_comment` 为 1 时该值有意义。 1 表不限制 2 表需审核' after `allow_comment`;

alter table `push_messages` add column `allow_comment` tinyint(1) not null default 1 comment '是否允许评论' after `content`;
alter table `push_messages` add column `limit_comment` tinyint not null default 1 comment '限制评论，当 `allow_comment` 为 1 时该值有意义。 1 表不限制 2 表需审核' after `allow_comment`;

create table if not exists `blogrolls` (
  `id` int unsigned not null auto_increment,
  `title` varchar(255) character set utf8mb4 collate utf8mb4_general_ci not null,
  `link` varchar(500) not null,
  `sort` smallint unsigned not null default 0,
  `icon` varchar(500) default null,
  `created_at` timestamp not null default current_timestamp ,
  `updated_at` timestamp null on update current_timestamp ,
  primary key (`id`)
) character set = utf8mb4 collate = utf8mb4_general_ci comment = '友情链接表';

alter table `subjects` add column `cover` varchar(500) default null comment '专题封面' after `name`;
alter table `contents` add column `cover` varchar(500) default null comment '内容封面' after `name`;
