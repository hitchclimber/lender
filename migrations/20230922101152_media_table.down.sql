-- Add down migration script here

drop type media_category;
drop table media, users, user_item_relation cascade;
