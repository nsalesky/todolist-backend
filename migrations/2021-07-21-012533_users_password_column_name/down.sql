-- This file should undo anything in `up.sql`
ALTER TABLE users
    RENAME COLUMN password TO password_hash;
