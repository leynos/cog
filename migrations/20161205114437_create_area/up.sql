CREATE TABLE area (
  id SERIAL PRIMARY KEY,
  garden_id INTEGER REFERENCES garden(id),
  name VARCHAR NOT NULL,
  UNIQUE(id, garden_id)
);
