#!/bin/sh

 qlx database create --database-url postgres://lender:lender@localhost:5232/lender_test_db
sqlx migrate add -r media_table
