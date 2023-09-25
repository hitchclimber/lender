-- Add up migration script here

create type media_category as enum ('book', 'dvd', 'blueray', 'vinyl', 'cd');

create extension if not exists "uuid-ossp";
create extension if not exists postgis;

-- TODO: add region/coordinates
-- NOTE:
-- ALTER TABLE table1 ADD COLUMN location GEOMETRY(point, 4326);
-- UPDATE table1 SET location = ST_SETSRID(ST_MakePoint(cast(longitude as float), cast(latitude as float)),4326);

create table users(
  id uuid primary key not null default (uuid_generate_v4()),
  first_name varchar(255) not null,
  last_name varchar(255) not null,
  email varchar(255) not null unique,
  password varchar(255) not null,
  created_at timestamp with time zone default now(),
  updated_at timestamp with time zone default now(),
  location geometry(point, 4326) not null
);

create table media (
  id uuid primary key,
  name varchar(255) not null,
  creator varchar(255) default 'anonymous' not null,
  year smallint not null,
  category varchar(255) not null,
  user_id uuid not null references users(id),
  available boolean default true not null
);

create table user_item_relation (
  user_id uuid references users(id),
  item_id uuid references media(id),
  primary key (user_id, item_id)
)
