-- Your SQL goes here
CREATE TABLE old_users (
       user_id SERIAL PRIMARY KEY,
       name VARCHAR NOT NULL,
       pwd VARCHAR NOT NULL,
       beer_count INT NOT NULL,
       shot_count INT NOT NULL,
       other_count INT NOT NULL,
       water_count INT NOT NULL,
       cigarette_count INT NOT NULL,
       fk_icon_id INT NOT NULL
);
