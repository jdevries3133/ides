create table comment(
    id serial primary key not null,
    comment text not null,

    block_id int not null references block(id),
    token_id int not null references token(id)
);
