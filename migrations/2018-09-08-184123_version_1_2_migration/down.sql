drop table if exists `discussants`;
drop table if exists `comments`;

alter table `contents` drop column `allow_comment`;
alter table `contents` drop column `limit_comment`;

alter table `push_messages` drop column `allow_comment`;
alter table `push_messages` drop column `limit_comment`;

drop table if exists `blogrolls`;