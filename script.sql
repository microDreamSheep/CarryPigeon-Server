create table "user"
(
    uuid     bigint  not null,
    username text    not null,
    password text    not null,
    status   varchar not null
);

create table "group"
(
    id     bigint,
    name   varchar,
    owner  json,
    admin  json,
    member json
);

create table private_message
(
    "from"     bigint  not null,
    "to"       bigint  not null,
    text       text,
    file_path  text,
    json       json,
    timestamp  varchar not null,
    message_id bigint
);

create table user_token
(
    uuid       bigint  not null,
    public_key varchar not null
);

create table group_message
(
    "from"     bigint  not null,
    group_id   bigint  not null,
    text       text,
    file_path  text,
    json       json,
    timestamp  varchar not null,
    message_id bigint
);


