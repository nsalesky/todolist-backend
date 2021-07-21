-- Your SQL goes here
CREATE TABLE user_lists (
    id SERIAL PRIMARY KEY,
    user_id INT references users (id) NOT NULL,
    list_id INT references lists (list_id) NOT NULL
);