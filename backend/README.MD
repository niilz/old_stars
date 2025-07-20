# How To Use Configurator

The configurator sets up the database schema and can add users, when passed to the application

## Environment-Variables

### user name and password

Set `PG_USER` and `pg_pwd`

Example in fish

```
set -x PG_USER some-user-name
set -x PG_PWD some-password
```

### dabase url

Set `DATABASE_URL`

Example in fish

```
set -x DATABASE_URL postgresql://some-user-name:the-password@public-address-of-ec2-instance:5432/db_name
```

Example for aws-url (make sure DB-Port accepts connections from this IP)
`postgresql://my-name:secret@ec2-1-42-43-424.eu-central-1.compute.amazonaws.com:5432/old_stars_db`

### Launch the configuration binary

`cargo run --bin configurator --user some-user --password some-password --role some-role`
`cargo run --bin configurator --user mega_admin --password top-secret --role admin`

### Disable Database

To disable the database set the environment variable `NO_DB` to `1`

Example in fish

```
set -x NO_DB 1
```

Note: Only the routes that do not use the database will be available
