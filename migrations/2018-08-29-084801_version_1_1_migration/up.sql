alter table `contents`
  add column `as_page` tinyint(1) not null default 0 comment '是否可以作为单页面' after `category_id`;

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