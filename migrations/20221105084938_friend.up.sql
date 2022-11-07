-- Add up migration script here
SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

-- ----------------------------
-- Table structure for friend
-- ----------------------------
DROP TABLE IF EXISTS `friend`;
CREATE TABLE `friend` (
  `fid` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '主键',
  `uid` bigint unsigned NOT NULL COMMENT '用户id',
  `friend_id` bigint unsigned NOT NULL COMMENT '好友id',
  `status` tinyint NOT NULL COMMENT '0 等待处理中, 1 同意, 2 拒绝',
  PRIMARY KEY (`fid`),
  UNIQUE KEY `uid_friend_id` (`uid`,`friend_id`) USING BTREE
) ENGINE=InnoDB AUTO_INCREMENT=4 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

SET FOREIGN_KEY_CHECKS = 1;
