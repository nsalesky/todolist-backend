-- Your SQL goes here
ALTER TABLE user_lists
    ADD COLUMN is_owner BOOLEAN NOT NULL DEFAULT TRUE;

ALTER TABLE user_lists
    ALTER COLUMN is_owner DROP DEFAULT;