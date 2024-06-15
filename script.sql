create table public."user"
(
    uuid     bigint  not null,
    username text    not null,
    password text    not null,
    status   varchar not null
);

create table public.user_token
(
    uuid       bigint  not null,
    public_key varchar not null
);

create table "group"."group"
(
    id     bigint,
    name   varchar,
    owner  bigint,
    admin  bigint[],
    member bigint[]
);

create table "group".group_message_template
(
    "from"     bigint,
    text       text,
    file_path  text,
    json       json,
    timestamp  varchar,
    message_id bigint
);

create table private_message.private_message_template
(
    "from"     bigint  not null,
    text       text,
    file_path  text,
    json       json,
    timestamp  varchar not null,
    message_id bigint
);


