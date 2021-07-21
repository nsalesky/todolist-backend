-- This file should undo anything in `up.sql`
ALTER TABLE users
    ALTER password_hash TYPE VARCHAR(50);
