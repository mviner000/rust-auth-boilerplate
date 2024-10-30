-- Your SQL goes here

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL UNIQUE,
    password VARCHAR NOT NULL,
    email VARCHAR NOT NULL UNIQUE
);

CREATE INDEX index_users_on_username ON users (username);
CREATE INDEX index_users_on_email ON users (email);

-- Password is 'password123' hashed with bcrypt
INSERT INTO users (username, password, email)
VALUES (
           'admin',
           '$2a$12$K0F5gKDxhVZL9P.zWKKJ2.BFtkFzHXTTi7O5yMoRgHmedE2uo5iWe',
           'admin@example.com'
       );

-- Verify the insert
SELECT id, username, email FROM users WHERE username = 'admin';