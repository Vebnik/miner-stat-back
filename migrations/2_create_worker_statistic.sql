create table "statistic"
(
    id              integer     primary key autoincrement,

    worker_id       integer     not null,

    mhs_av          integer     not null,
    temp            integer     not null,
    uptime          integer     not null,
    power           integer     not null,
    fan_in          integer     not null,
    fan_out         integer     not null,
    works           integer     not null,

    created_at      timestamp   not null default current_timestamp,
    updated_at      timestamp   not null default current_timestamp
);

create index "statistic_created_at" on "statistic" ("created_at");
create index "statistic_updated_at" on "statistic" ("updated_at");