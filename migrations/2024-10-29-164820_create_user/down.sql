-- This file should undo anything in `up.sql`

DROP INDEX IF EXISTS index_users_on_email;
DROP TABLE IF EXISTS users;
