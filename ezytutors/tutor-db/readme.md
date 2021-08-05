# About This Version

`tutor-db` is the version that uses PostgreSQL as the database for the persistence layer.

<br/>

## Setup

The setup (aka preparations before running any of the iterations mentioned below) mainly consists of having some environment variables and access to a database.

### Environment Variables

While being in this `tutor-db` directory, use `export PROJECT_ROOT=$(pwd)`.
This env var is referred by the scripts used in this scope.

<br/>

### Database Setup

Initially, a database and a user were created.

You can do it in the classic way:

- install or have access to a PostgreSQL server
- run the following SQL commands:
  ```sql
  create database eztutors;
  create user ezt with password 'ezpass';
  grant all privileges on database ezytutors to ezt;
  ```

or simpy run the included `./run_pgdb.sh` script that spins up a Docker container of a PostgreSQL server instance with these database and user ready to be used.

To create the data model and feed some initial data use `database.sql` script. As before, either:

- using a classic way with `psql` like this:<br/>
  `psql -U $DATABASE_USER -d ezytutors < $PROJECT_ROOT/src/database.sql`
- or using a GUI client (such as DBeaver).

## Iterations

- `./src/bin/iter1.rs` connects to the database and fetches the course with id 1.
