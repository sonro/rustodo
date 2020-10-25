CREATE TABLE tasks (
	id SERIAL PRIMARY KEY,
	title VARCHAR NOT NULL,
	done BOOLEAN NOT NULL DEFAULT 'f',
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    due TIMESTAMP,
	description TEXT
);

SELECT diesel_manage_updated_at('tasks');
