-- Your SQL goes here
CREATE TABLE accounts (
      id SERIAL PRIMARY KEY,
      user_id INTEGER NOT NULL UNIQUE REFERENCES users(id) ON DELETE CASCADE,
      first_name VARCHAR NULL,
      middle_name VARCHAR NULL,
      last_name VARCHAR NULL,
      avatar_300x300_url VARCHAR NULL,
      avatar_40x40_url VARCHAR NULL,
      created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
      updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX index_accounts_on_user_id ON accounts (user_id);
