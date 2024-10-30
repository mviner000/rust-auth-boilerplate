-- Your SQL goes here
CREATE TABLE accounts (
                          id SERIAL PRIMARY KEY,
                          user_id INTEGER NOT NULL UNIQUE REFERENCES users(id) ON DELETE CASCADE,
                          first_name VARCHAR,
                          middle_name VARCHAR,
                          last_name VARCHAR,
                          avatar_300x300_url VARCHAR NOT NULL,
                          avatar_40x40_url VARCHAR NOT NULL,
                          created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                          updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX index_accounts_on_user_id ON accounts (user_id);