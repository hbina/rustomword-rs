# rustomword

A play on random word, gives you a random word that you can use for ... whatever words are for.

For now, there is no real way to set it up locally.
You must deploy it to Heroku.
A live one is available here : https://rustomword.herokuapp.com/

# Setting Up Local Instance

## Watching

Use this command to watch,

`systemfd --no-pid -s http::5000 -- cargo watch -x run`

## Notes

Since Heroku's database URI are dynamic, there is no way to test local instance of this application against the official `rustomword`'s backend. Hence, you will have to provide the database yourself.

Thankfully, it's not *that* hard.

## Prerequisite

Currently, our setup is not database agnostic, so you will have to install PostgreSQL.

## Instructions

1. Log into `postgres` account on your machine using `sudo -i -u postgres`.
2. Execute `psql`.
3. Inside PostgresSQL terminal, create the user: `CREATE USER rustomword_test_user WITH PASSWORD '654321';`
4. Then, create the database: `CREATE DATABASE rustomword_test_db;`
5. Clone this repository: `https://github.com/hbina/english_dict_sql`
6. Get inside the cloned repository and execute: `psql -U rustomword_test_user -d rustomword_test_db < psql_entries.sql`
    1. You might encounter peer authentication error.
    2. If so, see Appendix A and redo 6.
7. Open `localhost:3000` in your browser.

# Appendix A

1. Using root privilege, open this file with your favorite text editor: `/etc/postgresql/10/main/pg_hba.conf`
2. Change this line:

```bash
# "local" is for Unix domain socket connections only
local   all             all                                     peer
```

to

```bash
# "local" is for Unix domain socket connections only
local   all             all                                     md5
```

