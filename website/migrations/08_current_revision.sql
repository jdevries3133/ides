insert into book (title) values ('default');
create table current_revision(
    revision_id int not null references book_revision(id),
    book_id int not null references book(id),

    unique (book_id),
    primary key (revision_id, book_id)
);

create table current_block(
    token_id int not null references token(id),
    block_id int not null references block(id),
    unique (token_id),
    primary key (token_id, block_id)
);
