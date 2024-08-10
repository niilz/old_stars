CREATE TABLE old_users (
	id SERIAL PRIMARY KEY,
	name VARCHAR NOT NULL,
	pwd VARCHAR NOT NULL,
	beer_count INT NOT NULL,
	shot_count INT NOT NULL,
	water_count INT NOT NULL,
  fk_icon_id INT NOT NULL
);
