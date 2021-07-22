-- This file should undo anything in `up.sql`
ALTER TABLE user_lists
    DROP CONSTRAINT user_lists_list_id_fkey,
    ADD CONSTRAINT user_lists_list_id_fkey
        FOREIGN KEY (list_id)
            REFERENCES lists(list_id);
