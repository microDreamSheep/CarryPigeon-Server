-- DROP SCHEMA public;

CREATE SCHEMA public AUTHORIZATION pg_database_owner;

COMMENT ON SCHEMA public IS 'standard public schema';
-- public.friend definition

-- Drop table

-- DROP TABLE public.friend;

CREATE TABLE public.friend (
	id int8 NOT NULL, -- 唯一id
	person_1 int8 NOT NULL, -- 发出好友申请的用户id
	person_2 int8 NOT NULL, -- 接受申请的好友id
	state int4 DEFAULT 0 NOT NULL, -- 状态值:1为已经通过，0为待处理，2为已拒绝
	application_time timestamptz DEFAULT now() NOT NULL, -- 发出申请时间
	CONSTRAINT friend_pk PRIMARY KEY (id)
);

-- Column comments

COMMENT ON COLUMN public.friend.id IS '唯一id';
COMMENT ON COLUMN public.friend.person_1 IS '发出好友申请的用户id';
COMMENT ON COLUMN public.friend.person_2 IS '接受申请的好友id';
COMMENT ON COLUMN public.friend.state IS '状态值:1为已经通过，0为待处理，2为已拒绝';
COMMENT ON COLUMN public.friend.application_time IS '发出申请时间';


-- public."group" definition

-- Drop table

-- DROP TABLE public."group";

CREATE TABLE public."group" (
	id int8 NOT NULL, -- 唯一id
	"name" varchar(40) NOT NULL, -- 群聊名字
	own_user_id int8 NOT NULL, -- 群聊创建者id
	"data" text DEFAULT '{}'::text NOT NULL, -- 群聊其他关联数据
	create_time timestamptz DEFAULT now() NOT NULL, -- 群聊创建时间
	CONSTRAINT group_pk PRIMARY KEY (id)
);

-- Column comments

COMMENT ON COLUMN public."group".id IS '唯一id';
COMMENT ON COLUMN public."group"."name" IS '群聊名字';
COMMENT ON COLUMN public."group".own_user_id IS '群聊创建者id';
COMMENT ON COLUMN public."group"."data" IS '群聊其他关联数据';
COMMENT ON COLUMN public."group".create_time IS '群聊创建时间';


-- public.group_member definition

-- Drop table

-- DROP TABLE public.group_member;

CREATE TABLE public.group_member (
	id int8 NOT NULL, -- 唯一id
	group_id int8 NOT NULL, -- 加入群聊的id
	user_id int8 NOT NULL, -- 群聊用户的名字
	"permission" int4 DEFAULT 3 NOT NULL, -- 用户权限，1：群主 2：管理员 3：普通用户
	state int4 DEFAULT 0 NOT NULL, -- 用户状态：1：待处理 2：已同意 3：已拒绝
	join_time timestamptz DEFAULT now() NOT NULL, -- 邀请发送时间
	CONSTRAINT group_member_pk PRIMARY KEY (id)
);

-- Column comments

COMMENT ON COLUMN public.group_member.id IS '唯一id';
COMMENT ON COLUMN public.group_member.group_id IS '加入群聊的id';
COMMENT ON COLUMN public.group_member.user_id IS '群聊用户的名字';
COMMENT ON COLUMN public.group_member."permission" IS '用户权限，1：群主 2：管理员 3：普通用户';
COMMENT ON COLUMN public.group_member.state IS '用户状态：1：待处理 2：已同意 3：已拒绝';
COMMENT ON COLUMN public.group_member.join_time IS '邀请发送时间';


-- public."user" definition

-- Drop table

-- DROP TABLE public."user";

CREATE TABLE public."user" (
	id int8 NOT NULL, -- 唯一主键，用雪花算法生成
	username varchar(40) NOT NULL, -- 用户名，最长为40个字符
	"password" varchar(64) NOT NULL, -- 密码，用sha256加密生成64字符长的哈希至
	"data" text DEFAULT '{}'::text NOT NULL, -- 用户数据，用json进行存储
	register_time timestamptz DEFAULT now() NULL, -- 注册时间
	CONSTRAINT user_id PRIMARY KEY (id)
);

-- Column comments

COMMENT ON COLUMN public."user".id IS '唯一主键，用雪花算法生成';
COMMENT ON COLUMN public."user".username IS '用户名，最长为40个字符';
COMMENT ON COLUMN public."user"."password" IS '密码，用sha256加密生成64字符长的哈希至';
COMMENT ON COLUMN public."user"."data" IS '用户数据，用json进行存储';
COMMENT ON COLUMN public."user".register_time IS '注册时间';

-- public.message definition

-- Drop table

-- DROP TABLE public.message;

CREATE TABLE public.message (
	id int8 NOT NULL, -- 唯一id
	from_id int8 NOT NULL, -- 消息发送人，指向一个用户
	to_id int8 NOT NULL, -- 发送到的位置
	message_tag int4 NOT NULL, -- 消息类型 1：群聊 2：私聊 3：树洞
	"data" text NOT NULL, -- 聊天数据，根据情况使用解释引擎解释
	message_type int4 DEFAULT 0 NOT NULL, -- 用于表示消息的类型
	"time" timestamptz DEFAULT now() NOT NULL, -- 消息发送时间
	CONSTRAINT message_pk PRIMARY KEY (id)
);

-- Column comments

COMMENT ON COLUMN public.message.id IS '唯一id';
COMMENT ON COLUMN public.message.from_id IS '消息发送人，指向一个用户';
COMMENT ON COLUMN public.message.to_id IS '发送到的位置';
COMMENT ON COLUMN public.message.message_tag IS '消息类型 1：群聊 2：私聊 3：树洞';
COMMENT ON COLUMN public.message."data" IS '聊天数据，根据情况使用解释引擎解释';
COMMENT ON COLUMN public.message.message_type IS '用于表示消息的类型';
COMMENT ON COLUMN public.message."time" IS '消息发送时间';