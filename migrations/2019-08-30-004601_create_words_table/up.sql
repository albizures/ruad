-- Your SQL goes here

CREATE TABLE words (
  id SERIAL PRIMARY KEY,
  word VARCHAR NOT NULL,
  counter INTEGER NOT NULL
)