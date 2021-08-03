## The Database version

This iteration uses PostgreSQL as the database for the persistence layer.

### Environment Variables

While being in this `tutor-db` directory, use `export PROJECT_ROOT=$(pwd)`.
This env var is referred by the scripts used in this scope.

### DB Setup

Initially, a database and user were created. You can do it in the classic way:

```sql
create database eztutors;
create user ezt with password 'ezpass';
grant all privileges on database ezytutors to ezt;
```

or simpy run the included `./run_pgdb.sh` script that spins up a Docker containers with PostgreSQL.

To create the data model and feed some initial data use `database.sql` script. Again, either using a classic way with `psql` like this:<br/>
`psql -U $DATABASE_USER -d ezytutors < $PROJECT_ROOT/src/database.sql`<br/>
or using a GUI client/tool such as DBeaver, if you prefer.
