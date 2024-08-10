-- Your SQL goes here
CREATE TABLE roles (
	id SERIAL PRIMARY KEY,
	user_id INT NOT NULL,
	role VARCHAR NOT NULL
);
