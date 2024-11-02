-- Your SQL goes here
CREATE TABLE messages (
      id SERIAL PRIMARY KEY,
      sender_id INTEGER NOT NULL,
      receiver_id INTEGER NOT NULL,
      content TEXT NOT NULL,
      is_read BOOLEAN NOT NULL DEFAULT FALSE,
      created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
      CONSTRAINT fk_sender
          FOREIGN KEY(sender_id)
              REFERENCES users(id)
              ON DELETE CASCADE,
      CONSTRAINT fk_receiver
          FOREIGN KEY(receiver_id)
              REFERENCES users(id)
              ON DELETE CASCADE
);

-- Create indexes for better query performance
CREATE INDEX idx_message_users ON messages(sender_id, receiver_id);
CREATE INDEX idx_message_timestamp ON messages(created_at);