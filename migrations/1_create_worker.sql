create table "worker"
(
    id              integer     primary key autoincrement,

    name            text        not null,
    port            text        not null,
    host            text        not null,

    created_at      timestamp   not null default current_timestamp,
    updated_at      timestamp   not null default current_timestamp
);

create index "worker_created_at" on "worker" ("created_at");
create index "worker_updated_at" on "worker" ("updated_at");