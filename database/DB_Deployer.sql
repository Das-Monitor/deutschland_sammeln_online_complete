CREATE USER creation_manager WITH PASSWORD 'HNB69tAcpcLg6d#'

CREATE TABLE user_credentials (
    id SERIAL NOT NULL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL
);

ALTER TABLE user_credentials OWNER TO postgres;

GRANT SELECT ON user_credentials TO credential_manager;

CREATE TABLE user_sessions (
    id SERIAL NOT NULL PRIMARY KEY,
    username TEXT NOT NULL,
    session_token TEXT NOT NULL,
    creation_timestamp TIMESTAMP NOT NULL,
    UNIQUE (username, session_token)
);

ALTER TABLE user_sessions OWNER TO postgres;

GRANT SELECT, INSERT, UPDATE ON user_sessions TO credential_manager;

GRANT SELECT ON gemeinden TO credential_manager;

GRANT USAGE ON ALL SEQUENCES IN SCHEMA public TO credential_manager;

CREATE USER deutschland_sammeln_bot WITH PASSWORD 'wHz6ghG6-Fa$8uu3';

CREATE TABLE gemeinden (
    id SERIAL PRIMARY KEY NOT NULL ,
    gemeinde_name TEXT NOT NULL,
    occasion TEXT NOT NULL,
    entry_timestamp TIMESTAMP NOT NULL,
    UNIQUE(gemeinde_name, occasion)
);

ALTER TABLE gemeinden owner to postgres;
grant insert, select, update on gemeinden to deutschland_sammeln_bot;

CREATE TABLE gemeinde_entry (
    id SERIAL PRIMARY KEY NOT NULL,
    gemeinde_name TEXT NOT NULL,
    occasion TEXT NOT NULL,
    entry_time TIMESTAMP NOT NULL,
    UNIQUE(gemeinde_name, occasion, entry_time),
    FOREIGN KEY (gemeinde_name, occasion) REFERENCES gemeinden(gemeinde_name, occasion)
);

ALTER TABLE gemeinde_entry owner to postgres;
grant insert, select, update on gemeinde_entry to deutschland_sammeln_bot;

CREATE TABLE gemeinde_result (
    id SERIAL PRIMARY KEY NOT NULL,
    gemeinde_name TEXT NOT NULL,
    occasion TEXT NOT NULL,
    UNIQUE(gemeinde_name, occasion),
    FOREIGN KEY (gemeinde_name, occasion) REFERENCES gemeinden(gemeinde_name, occasion)
);

ALTER TABLE gemeinde_result owner to postgres;
grant insert, select, update on gemeinde_result to deutschland_sammeln_bot;


grant usage, select ON ALL SEQUENCES IN SCHEMA public TO deutschland_sammeln_bot;

