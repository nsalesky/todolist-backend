-- This file should undo anything in `up.sql`
ALTER TABLE users
    ADD COLUMN login_session VARCHAR NOT NULL;
