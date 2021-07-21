-- Your SQL goes here
ALTER TABLE users
    RENAME COLUMN password_hash TO password;
