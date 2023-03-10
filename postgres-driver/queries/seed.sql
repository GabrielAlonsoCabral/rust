DROP TABLE IF EXISTS Author;

CREATE TABLE Author (
    id              SERIAL PRIMARY KEY,
    name            VARCHAR NOT NULL,
    country         VARCHAR NOT NULL
);

INSERT INTO Author ("id", "name", "country")
SELECT
	x.id,
    'Gabriel Alonso',
    'Brazil'
  FROM generate_series(1,100000) AS x(id);