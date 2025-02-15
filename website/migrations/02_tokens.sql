create table role(
    id serial primary key not null,
    name text not null
);

insert into role (name) values
    ('reader'),
    ('admin')
;

create table token(
    id serial primary key not null,
    token text not null,
    name text not null,

    role_id int references role(id) not null
);

create index idx_token on token(token);
