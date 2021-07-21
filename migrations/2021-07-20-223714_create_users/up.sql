CREATE TABLE users
(
    id       SERIAL      PRIMARY KEY,
    username      VARCHAR(30) NOT NULL UNIQUE,
    email         VARCHAR(30) NOT NULL UNIQUE,
    name          VARCHAR(50) NOT NULL,
    password_hash VARCHAR(50) NOT NULL
);
