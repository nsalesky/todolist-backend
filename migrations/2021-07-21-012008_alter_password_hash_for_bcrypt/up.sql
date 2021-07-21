-- Your SQL goes here
ALTER TABLE users
    ALTER password_hash TYPE CHAR(60);
