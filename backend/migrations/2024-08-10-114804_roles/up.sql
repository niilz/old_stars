-- Your SQL goes here
CREATE TABLE roles (
	role_id SERIAL PRIMARY KEY,
	fk_user_id INTEGER NOT NULL REFERENCES old_users(user_id),
	role VARCHAR NOT NULL
);
