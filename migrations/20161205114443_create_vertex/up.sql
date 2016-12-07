CREATE TABLE vertex (
  id SERIAL PRIMARY KEY,
  area_id INTEGER REFERENCES area(id),
  x FLOAT NOT NULL,
  y FLOAT NOT NULL,
  ordinal FLOAT NOT NULL,
  UNIQUE(area_id, ordinal)
);
