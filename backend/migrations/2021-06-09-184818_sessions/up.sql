create TABLE sessions (
	id SERIAL PRIMARY KEY,
	name VARCHAR NOT NULL,
	value VARCHAR NOT NULL,
  user_id INT NOT NULL
)
