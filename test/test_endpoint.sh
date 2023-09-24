#!/bin/sh

curl -X POST  -H 'Content-Type: application/json' http://localhost:8080/api/users/user \
  -d '{
  "first_name":"Andreas",
  "last_name":"Schmid",
  "email":"hello@mail.ch", 
  "password":"dadada"
}'

curl -X GET http://localhost:8080/api/users

curl -X GET http://localhost:8080/api/users/user/ced9a468-d4d0-4838-867a-7a257f78ec08

curl -X PUT  -H 'Content-Type: application/json' http://localhost:8080/api/users/user/ced9a468-d4d0-4838-867a-7a257f78ec08 \
  -d '{
  "first_name":"Andreas",
  "last_name":"Schmid",
  "email": "new_address@mailer.com"
}'


