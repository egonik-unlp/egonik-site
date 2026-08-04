-- Your SQL goes here
--
CREATE TABLE IF NOT EXISTS job_experiences (
   id serial primary key,
   date_from date not null,
   date_to date ,
   job_title varchar(255) not null,
   accomplishments varchar(255) not null,
   responsabilities varchar(255) not null
);

create table if not exists job_institutions (
  id serial primary key,
  job_experience_id serial,
  name varchar(255) not null,
  url varchar(255) not null,
  foreign keY(JOB_EXPERIENCE_ID) references JOB_EXPERIENCES(ID)
);

