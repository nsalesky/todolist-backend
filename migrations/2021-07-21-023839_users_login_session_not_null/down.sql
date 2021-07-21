-- This file should undo anything in `up.sql`
ALTER TABLE users
    ALTER COLUMN login_session DROP NOT NULL;