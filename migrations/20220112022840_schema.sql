create table if not exists blossom.accounts
(
    id                 serial primary key unique                not null,
    email              text,
    password_hash      text                                     not null,
    roles              varchar(16)[] default array[]::varchar[] not null,

    /* Meta */
    created_on         timestamptz   default now()              not null,
    modified_on        timestamptz   default now()              not null,
    deleted            boolean       default false              not null
);

create table if not exists blossom.players
(
    id          serial primary key unique not null,
    account_id  int                       not null,
    name        varchar(16) unique        not null,
    position    int[] default '{0, 0, 0}' not null,
    health      int default 100           not null,
    max_health  int default 100           not null,
    mana        int default 100           not null,
    max_mana    int default 100           not null,
    xp          int default 0             not null,
    level       int default 1             not null,
    brief       boolean default false     not null,
    afk         boolean default false     not null,

    /* Constraints */
    constraint fk_account foreign key (account_id) references accounts (id),

    /* Meta */
    created_on  timestamptz default now() not null,
    modified_on timestamptz default now() not null,
    deleted     boolean     default false not null
);

create table if not exists blossom.tickets
(
    id          serial primary key unique not null,
    author      varchar(16)               not null,
    description text                      not null,

    /* Constraints */
    constraint fk_author foreign key (author) references players (name),

    /* Meta */
    created_on  timestamptz default now() not null,
    modified_on timestamptz default now() not null,
    deleted     boolean     default false not null
);

create table if not exists blossom.action_logs
(
    action_log_id  serial primary key,
    account_id     int references accounts (id),
    ip_address     inet                         not null,
    kind           varchar(255)                 not null,
    detail         text,
    created_on     timestamptz default now()    not null
);

create or replace function update_modified_on()
    returns trigger as
$$ begin
        new.modified_on := now();

        return new;
end; $$ language plpgsql;

drop trigger if exists on_update_account ON "blossom"."accounts";

create trigger on_modify_account
    before insert or update
    on accounts
    for each row
execute procedure update_modified_on();

drop trigger if exists on_update_account ON "blossom"."players";

create trigger on_modify_account
    before insert or update
    on players
    for each row
execute procedure update_modified_on();

drop trigger if exists on_update_account ON "blossom"."tickets";

create trigger on_modify_account
    before insert or update
    on tickets
    for each row
execute procedure update_modified_on();