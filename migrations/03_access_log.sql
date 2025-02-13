create table access_log(
    id serial primary key not null,
    created_at timestamp with time zone not null default now(),
    page int not null,

    token_id int not null references token(id)
);
