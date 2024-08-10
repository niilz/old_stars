-- Your SQL goes here
CREATE TABLE roles (
	id SERIAL PRIMARY KEY,
	user_id INTEGER NOT NULL REFERENCES old_users(id),
	role VARCHAR NOT NULL
);
