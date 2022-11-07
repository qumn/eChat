-- Add up migration script here
CREATE TABLE `message` (
  `mid` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '消息id',
  `content` text COMMENT '消息内容',
  `sender_uid` bigint unsigned DEFAULT NULL COMMENT '发送者id',
  `receiver_type` tinyint DEFAULT NULL COMMENT '接收对象类型 0 群 1 用户 ',
  `receiver_id` bigint unsigned DEFAULT NULL COMMENT '接受对象id',
  `create_time` datetime DEFAULT NULL COMMENT '创建时间',
  PRIMARY KEY (`mid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;