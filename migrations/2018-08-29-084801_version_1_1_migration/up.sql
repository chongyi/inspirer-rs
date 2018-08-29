alter table `contents`
  add column `as_page` tinyint(1) not null default 0 comment '是否可以作为单页面' after `category_id`;