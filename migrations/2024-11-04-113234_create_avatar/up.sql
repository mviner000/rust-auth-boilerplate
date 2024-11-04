-- Your SQL goes here
CREATE TABLE avatars (
                         id SERIAL PRIMARY KEY,
                         account_id INTEGER NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
                         avatar_300x300_url VARCHAR NULL,
                         avatar_40x40_url VARCHAR NULL,
                         created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                         updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX index_avatars_on_account_id ON avatars (account_id);

ALTER TABLE accounts
    ADD COLUMN default_avatar_id INTEGER NULL REFERENCES avatars(id) ON DELETE SET NULL;

CREATE INDEX index_accounts_on_default_avatar_id ON accounts (default_avatar_id);