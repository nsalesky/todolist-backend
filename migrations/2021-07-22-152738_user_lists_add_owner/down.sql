-- This file should undo anything in `up.sql`
ALTER TABLE user_lists
    DROP COLUMN is_owner;