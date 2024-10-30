-- Your SQL goes here

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL UNIQUE,
    password VARCHAR NOT NULL,
    email VARCHAR NOT NULL UNIQUE
);

CREATE INDEX index_users_on_username ON users (username);
CREATE INDEX index_users_on_email ON users (email);
