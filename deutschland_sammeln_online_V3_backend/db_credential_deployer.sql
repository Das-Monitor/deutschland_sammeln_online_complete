CREATE TABLE user_credentials (
    id SERIAL NOT NULL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL
);

ALTER TABLE user_credentials OWNER TO admin;

GRANT SELECT ON user_credentials TO credential_manager;

CREATE TABLE user_sessions (
    id SERIAL NOT NULL PRIMARY KEY,
    username TEXT NOT NULL,
    session_token TEXT NOT NULL,
    creation_timestamp TIMESTAMP NOT NULL,
    UNIQUE (username, session_token)
);

ALTER TABLE user_sessions OWNER TO admin;

GRANT SELECT, INSERT, UPDATE ON user_sessions TO credential_manager;

GRANT SELECT ON gemeinden TO credential_manager;

GRANT USAGE ON ALL SEQUENCES IN SCHEMA public TO credential_manager;