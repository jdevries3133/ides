drop table block;
drop table block_type;
drop table book;

create table book(
    id serial primary key not null,
    title text not null
);

create table book_revision(
    id serial primary key not null,
    created_at timestamp with time zone not null default now(),

    book_id int not null references book(id)
);

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

create table block(
    id serial primary key not null,
    created_at timestamp with time zone not null default now(),
    sequence int not null,
    content text not null,
    content_checksum text generated always as (md5(content)) stored,

    type_id int not null references block_type(id),
    book_revision_id int not null references book_revision(id)
);
