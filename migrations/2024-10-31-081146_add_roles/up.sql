-- Your SQL goes here
CREATE TABLE roles (
                       id SERIAL PRIMARY KEY,
                       name VARCHAR(50) NOT NULL UNIQUE,
                       description VARCHAR(255),
                       created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                       updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX index_roles_on_name ON roles (name);

CREATE TABLE user_roles (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role_id INTEGER NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(user_id, role_id)
);

CREATE INDEX index_user_roles_on_user_id ON user_roles (user_id);
CREATE INDEX index_user_roles_on_role_id ON user_roles (role_id);

-- Insert initial roles
INSERT INTO roles (name, description) VALUES
      ('superuser', 'System administrator with full access'),
      ('admin', 'User with elevated privileges for content moderation'),
      ('user', 'Regular user with standard privileges');