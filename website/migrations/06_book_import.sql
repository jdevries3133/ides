alter table paragraph rename to block;

create table block_type(
    id serial primary key not null,
    name text not null
);

insert into block_type (name)
values
    ('paragraph'),
    ('h1'),
    ('h2'),
    ('section title')
;

alter table block add column type_id int not null references block_type(id);
alter table book drop column revision;
alter table book add column created_at timestamp with time zone not null default now();
