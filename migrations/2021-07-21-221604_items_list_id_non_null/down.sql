-- This file should undo anything in `up.sql`
ALTER TABLE items
    ALTER COLUMN list_id DROP NOT NULL;
