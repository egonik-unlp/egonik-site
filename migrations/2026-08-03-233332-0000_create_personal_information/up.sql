-- Your SQL goes here
create table if not exists personal_informations (
  id serial primary key,
  name varchar(255) not null,
  surname varchar(255) not null,
  image_url varchar(255) not null,
  birth_date date not null
);

create table if not exists contact_informations (
  id serial primary key,
  personal_information_id serial not null,
  github varchar(255) not null,
  email varchar(255) not null,
  instagram varchar(255) not null,
  linked_in varchar(255) not null,
  foreign keY(personal_information_id) references personal_informations(id)
);
