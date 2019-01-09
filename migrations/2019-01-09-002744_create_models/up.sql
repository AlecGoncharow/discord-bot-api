-- Your SQL goes here
CREATE TABLE users (
	id BIGINT PRIMARY KEY NOT NULL,
	lifetime_gross INT DEFAULT 0,
	lifetime_net INT DEFAULT 0,
	week_gross INT DEFAULT 0,
	week_net INT DEFAULT 0,
	tips INT DEFAULT 0,
	tips_given INT DEFAULT 0,
	anti_tips INT DEFAULT 0,
	anti_tips_given INT DEFAULT 0
);

CREATE TABLE tips (
	id SERIAL PRIMARY KEY,
	user_from BIGINT NOT NULL,
	user_to BIGINT NOT NULL,
	time BIGINT NOT NULL,
	anti BOOLEAN NOT NULL DEFAULT 'f'
);
