-- Your SQL goes here
CREATE TABLE lists (
    list_id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    date_created DATE NOT NULL
);
