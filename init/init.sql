CREATE TABLE agents (
    id SERIAL PRIMARY KEY,
    code VARCHAR(100) NOT NULL UNIQUE,
    display_name VARCHAR(100) NOT NULL,
    system_prompt TEXT NOT NULL
);
