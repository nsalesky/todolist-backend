-- This file should undo anything in `up.sql`
ALTER TABLE users
    RENAME COLUMN password_hash to password;

ALTER TABLE users
    RENAME COLUMN preferred_name to name;
