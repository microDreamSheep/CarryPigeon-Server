create table "user"
(
    uuid     uuid    not null,
    username text    not null,
    password text    not null,
    status   varchar not null
);

create table "group"
(
    id     uuid,
    name   varchar,
    owner  json,
    admin  json,
    member json
);

create table private_temp_message
(
    "from"    uuid      not null,
    "to"      uuid      not null,
    text      text,
    file_path text,
    json      json,
    timestamp timestamp not null
);


