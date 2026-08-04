-- Your SQL goes here
--
--
--
CREATE TABLE if not exists portfolio_items (
  id SERIAL PRIMARY KEY,
  title VARCHAR(255) unique not null,
  description TEXT not null,
  public BOOLEAN not null,
  public_url VARCHAR(255)
);

create table if not exists tags (
id serial primary key,
portfolio_item_id serial not null,
value varchar(255) not null,
FOREIGN KEY(portfolio_item_id) REFERENCES portfolio_items(id)
);
