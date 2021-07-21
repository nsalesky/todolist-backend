-- Your SQL goes here
ALTER TABLE users
    RENAME COLUMN password to password_hash;

ALTER TABLE users
    RENAME COLUMN name to preferred_name;