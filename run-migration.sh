#!/bin/sh

sqlx database create --database-url postgres://lender:lender@localhost:5232/lender_postgis
sqlx migrate add -r media_table
