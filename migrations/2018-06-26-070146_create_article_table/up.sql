CREATE TABLE `articles` (
    `id` INT(10) UNSIGNED NOT NULL AUTO_INCREMENT,
    `title` VARCHAR(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
    `category_id` INT(11) UNSIGNED NOT NULL,
    `keywords` VARCHAR(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
    `description` VARCHAR(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
    `content` MEDIUMTEXT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
    `sort` SMALLINT(4) UNSIGNED NOT NULL DEFAULT '0',
    `name` VARCHAR(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci DEFAULT NULL,
    `views` INT(11) UNSIGNED NOT NULL DEFAULT '0',
    `display` SMALLINT(1) UNSIGNED NOT NULL DEFAULT '1',
    `modified_at` TIMESTAMP NULL COMMENT '最后修改时间',
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`),
    UNIQUE KEY `union_name` (`name`),
    KEY `index_title` (`title`),
    KEY `index_category` ('category_id')
)