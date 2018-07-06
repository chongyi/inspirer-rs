CREATE TABLE IF NOT EXISTS `users` (
    `id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
    `name` VARCHAR(60) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
    `email` VARCHAR(160) NOT NULL,
    `password` VARCHAR(160) DEFAULT NULL,
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`),
    UNIQUE KEY `union_email` (`email`),
    KEY `index_name` (`name`)
);

CREATE TABLE IF NOT EXISTS `categories` (
    `id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
    `name` VARCHAR(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
    `display_name` VARCHAR(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
    `description` VARCHAR(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
    `sort` SMALLINT(4) NOT NULL DEFAULT '0',
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`),
    UNIQUE KEY `union_name` (`name`)
) CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `contents` (
    `id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
    `creator_id` INT(11) UNSIGNED NOT NULL,
    `title` VARCHAR(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
    `category_id` INT(11) UNSIGNED DEFAULT NULL,
    `keywords` VARCHAR(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
    `description` VARCHAR(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
    `sort` SMALLINT(4) UNSIGNED NOT NULL DEFAULT '0',
    `display` TINYINT(1) NOT NULL DEFAULT '1',
    `content_type` SMALLINT UNSIGNED NOT NULL DEFAULT '1' COMMENT '内容类型，1 表文章',
    `content_id` INT(11) UNSIGNED NOT NULL,
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`),
    KEY `index_title` (`title`),
    KEY `index_category` (`category_id`)
) CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `content_articles` (
    `id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
    `content_id` INT(11) UNSIGNED DEFAULT NULL,
    `content` MEDIUMTEXT CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
    `name` VARCHAR(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT NULL,
    `views` INT(11) UNSIGNED NOT NULL DEFAULT '0',
    `modified_at` TIMESTAMP NULL COMMENT '最后修改时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `union_name` (`name`)
) CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci;