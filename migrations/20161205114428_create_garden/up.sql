CREATE TABLE garden (
  id SERIAL PRIMARY KEY,
  name VARCHAR UNIQUE NOT NULL,
  description VARCHAR NOT NULL
);