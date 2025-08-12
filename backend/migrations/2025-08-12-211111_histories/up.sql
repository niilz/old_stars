-- Your SQL goes here
CREATE TABLE history (
	history_id SERIAL PRIMARY KEY,
	user_name VARCHAR NOT NULL,
	timestamp TIMESTAMP NOT NULL,
	beer_count INT NOT NULL,
	shot_count INT NOT NULL,
	other_count INT NOT NULL,
	water_count INT NOT NULL,
    cigarette_count INT NOT NULL
);
