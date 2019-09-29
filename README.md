# rustomword

A play on random word, gives you a random word that you can use for ... whatever words are for.

For now, there is no real way to set it up locally.
You must deploy it to Heroku.
A live one is available here : https://rustomword.herokuapp.com/

# Setting Up Local Instance

## Notes

Since Heroku's database URI are dynamic, there is no way to test local instance of this application against the `rustomword`'s backend. Hence, you will have to provide the database yourself.

Thankfully, it's not *that* hard.

## Prerequisite

Currently, our setup is not database agnostic.
You will have to install PostgreSQL.

## Instructions

1. Access the database, and create a user using the name `rustomword_test` and the password `654321`. This is just for local testing, so I thought its fine to expose them here :3
2. Create a database with the name `rustomword_test`.
3. Clone this repository : https://github.com/hbina/english_dict_sql
4. You will then have to feed the database with the `entries` table which is provided by the SQL file. To do so, simply execute : `psql -d rustomword < psql_entries.sql`
5. Execute `cargo run`. You will encounter "errors" saying some environment variables are not set up, but you can ignore them.
