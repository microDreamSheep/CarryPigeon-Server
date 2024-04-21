create table "user"
(
    uuid     uuid    not null,
    username varchar not null,
    password varchar not null
);

create table "group"
(
    id     uuid,
    name   varchar,
    owner  json,
    admin  json,
    member json
);

create table temp_message
(
    "from" uuid not null,
    "to"   uuid not null
);


