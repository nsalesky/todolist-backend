-- This file should undo anything in `up.sql`
ALTER TABLE items
    DROP CONSTRAINT items_list_id_fkey,
    ADD CONSTRAINT items_list_id_fkey
        FOREIGN KEY (list_id)
            REFERENCES lists(list_id);