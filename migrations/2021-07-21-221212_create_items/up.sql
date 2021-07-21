-- Your SQL goes here
CREATE TABLE items (
   item_id SERIAL PRIMARY KEY,
   list_id INT REFERENCES lists (list_id),
   description TEXT NOT NULL,
   finished BOOLEAN NOT NULL DEFAULT FALSE

);