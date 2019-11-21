-- Your SQL goes here
-- probably bad form but :)
CREATE TABLE times (
	id SERIAL PRIMARY KEY NOT NULL,
	last_reset_time BIGINT NOT NULL DEFAULT 0
);
