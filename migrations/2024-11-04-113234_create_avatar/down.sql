-- This file should undo anything in `up.sql`
-- First remove the default_avatar_id foreign key from accounts
ALTER TABLE accounts
DROP COLUMN IF EXISTS default_avatar_id;

-- Then drop the avatars table
DROP TABLE IF EXISTS avatars;