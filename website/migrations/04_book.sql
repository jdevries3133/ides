create table book(
    id serial primary key not null,
    title text not null,
    revision text not null
);

create table paragraph(
    id serial primary key not null,
    created_at timestamp with time zone not null default now(),
    sequence int not null,
    content text not null,
    content_checksum text generated always as (md5(content)) stored,

    book_id int not null references book(id)
);

