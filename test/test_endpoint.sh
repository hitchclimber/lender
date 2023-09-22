#!/bin/sh

curl -X POST -d '{
  "id": "kNumiocd2ATdC0KAH9fq",
  "first_name":"Albert",
  "last_name":"Schweitzer",
  "email":"as@mail.ch", 
  "password":"hashed",
}' -H "Content-Type: application/json" http://localhost:8000/users
