-- This file should undo anything in `up.sql`
DROP INDEX IF EXISTS index_user_roles_on_role_id;
DROP INDEX IF EXISTS index_user_roles_on_user_id;
DROP INDEX IF EXISTS index_roles_on_name;

DROP TABLE IF EXISTS user_roles;
DROP TABLE IF EXISTS roles;