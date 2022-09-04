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

`cargo run --bin configurator some-user some-password`
`cargo run --bin configurator mega_admin top-secret`
