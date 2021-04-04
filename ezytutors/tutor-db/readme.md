## The Database version

This iteration uses PostgreSQL as the database for the persistence layer.

### Environment Variables

While being in this `tutor-db` directory, use `export PROJECT_ROOT=$(pwd)` to be considered by the scripts used in this scope.

### DB Setup

Initially, a database and user were created:

```sql
create database ezdb;

create user ezt with password 'ezpass';

```
