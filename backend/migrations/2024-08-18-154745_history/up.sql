-- Your SQL goes here
CREATE TABLE history (
	history_id SERIAL PRIMARY KEY,
	user_id INTEGER NOT NULL REFERENCES old_users(user_id),
	user_name VARCHAR NOT NULL,
	timestamp TIMESTAMP NOT NULL,
	beer_count INT NOT NULL,
	shot_count INT NOT NULL,
	other_count INT NOT NULL,
	water_count INT NOT NULL
);
