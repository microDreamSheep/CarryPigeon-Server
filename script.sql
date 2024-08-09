
CREATE TABLE friend (
    id int8 NOT NULL comment "唯一id",
    person_1 int8 NOT NULL comment "发出好友申请的用户id",
    person_2 int8 NOT NULL comment "接受申请的好友id",
    state int4 DEFAULT 0 NOT NULL comment "状态值:1为已经通过，0为待处理，2为已拒绝",
    application_time timestamp DEFAULT now() NOT NULL comment "发出申请时间",
    CONSTRAINT friend_pk PRIMARY KEY (id)
);

CREATE TABLE 'group' (
     id int8 NOT NULL comment "唯一id",
     "name" varchar(40) NOT NULL comment "群聊名字",
     own_user_id int8 NOT NULL comment "群聊创建者id",
     "data" text DEFAULT '{}' NOT NULL comment "群聊其他关联数据",
     create_time timestamp DEFAULT now() NOT NULL comment "群聊创建时间",
     CONSTRAINT group_pk PRIMARY KEY (id)
);

CREATE TABLE group_member (
      id int8 NOT NULL comment "唯一id",
      group_id int8 NOT NULL comment "加入群聊的id",
      user_id int8 NOT NULL comment "群聊用户的名字",
      "permission" int4 DEFAULT 3 NOT NULL comment "用户权限，1：群主 2：管理员 3：普通用户",
      state int4 DEFAULT 0 NOT NULL comment "用户状态：1：待处理 2：已同意 3：已拒绝",
      application_time timestamp DEFAULT now() NOT NULL comment "邀请发送时间",
      CONSTRAINT group_member_pk PRIMARY KEY (id)
);

CREATE TABLE "user" (
    id int8 NOT NULL comment "唯一主键，用雪花算法生成",
    username varchar(40) NOT NULL comment "用户名，最长为40个字符",
    "password" varchar(64) NOT NULL comment "密码，用sha256加密生成64字符长的哈希至",
    "data" text DEFAULT '{}' NOT NULL comment "用户数据，用json进行存储",
    register_time timestamp DEFAULT now() NULL comment "注册时间",
    CONSTRAINT user_id PRIMARY KEY (id)
);

CREATE TABLE message (
     id int8 NOT NULL comment "唯一id",
     from_id int8 NOT NULL comment "消息发送人，指向一个用户",
     to_id int8 NOT NULL comment "发送到的位置",
     message_tag int4 NOT NULL comment "消息类型 1：群聊 2：私聊 3：树洞",
     "data" text NOT NULL comment "聊天数据，根据情况使用解释引擎解释",
     message_type int4 DEFAULT 0 NOT NULL comment "用于表示消息的类型，0为文本类型",
     "time" timestamp DEFAULT now() NOT NULL comment "消息发送时间",
     CONSTRAINT message_pk PRIMARY KEY (id)
);
