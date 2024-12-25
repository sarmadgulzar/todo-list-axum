-- Add up migration script here
CREATE TABLE IF NOT EXISTS todos (
    id INTEGER PRIMARY KEY NOT NULL,
    title VARCHAR(250) NOT NULL,
    completed BOOLEAN NOT NULL
);