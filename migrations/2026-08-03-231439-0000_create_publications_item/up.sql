-- Your SQL goes here
--
create table if not exists publication_items (
  id serial primary key,
  title varchar(255) not null unique,
  abs text not null,
  year int not null,
  journal varchar(255) not null,
  link varchar(400) not null

)
