-- Add up migration script here
DROP TABLE IF EXISTS `group`;
CREATE TABLE `group` (
    `gid` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '群id',
    `name` varchar(20) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '群名称',
    `create_time` datetime NOT NULL COMMENT '创建时间',
    `owner` bigint unsigned NOT NULL COMMENT '群主id',
    PRIMARY KEY (`gid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

DROP TABLE IF EXISTS `group_user`;
CREATE TABLE `group_user` (
    `guid` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '主键',
    `gid` bigint unsigned DEFAULT NULL COMMENT '群id',
    `uid` bigint unsigned DEFAULT NULL COMMENT '用户id',
    `status` tinyint DEFAULT NULL COMMENT '0 群主 1 管理员 2 普通成员',
    PRIMARY KEY (`guid`),
    UNIQUE KEY `uid_gid_idx` (`uid`,`gid`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;