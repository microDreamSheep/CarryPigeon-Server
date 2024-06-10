create table "group"
(
    id     bigint,
    name   varchar,
    owner  bigint,
    admin  bigint[],
    member bigint[]
);

create table group_message_template
(
    "from"     bigint,
    text       text,
    file_path  text,
    json       json,
    timestamp  varchar,
    message_id bigint
);


