-- Your SQL goes here
ALTER TABLE user_lists
    DROP CONSTRAINT user_lists_user_id_fkey,
    ADD CONSTRAINT user_lists_user_id_fkey
        FOREIGN KEY (user_id)
            REFERENCES users(id)
            ON DELETE CASCADE;
