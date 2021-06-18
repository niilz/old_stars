CREATE TABLE old_users (
	id SERIAL PRIMARY KEY,
	name VARCHAR NOT NULL,
	salt VARCHAR NOT NULL,
	pwd VARCHAR NOT NULL,
	beer_count INT,
	shot_count INT,
	water_count INT,
  fk_icon_id INT
);
