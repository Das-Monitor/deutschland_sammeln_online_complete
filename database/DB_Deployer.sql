CREATE USER deutschland_sammeln_bot WITH PASSWORD 'wHz6ghG6-Fa$8uu3' CONNECT;

CREATE TABLE gemeinden (
    id SERIAL PRIMARY KEY NOT NULL ,
    gemeinde_name TEXT NOT NULL,
    occasion TEXT NOT NULL,
    entry_timestamp TIMESTAMP NOT NULL,
    UNIQUE(gemeinde_name, occasion)
);

ALTER TABLE gemeinden owner to admin;
grant insert, select, update on gemeinden to deutschland_sammeln_bot;

CREATE TABLE gemeinde_entry (
    id SERIAL PRIMARY KEY NOT NULL,
    gemeinde_name TEXT NOT NULL,
    occasion TEXT NOT NULL,
    entry_time TIMESTAMP NOT NULL,
    UNIQUE(gemeinde_name, occasion, entry_time),
    FOREIGN KEY (gemeinde_name, occasion) REFERENCES gemeinden(gemeinde_name, occasion)
);

ALTER TABLE gemeinde_entry owner to admin;
grant insert, select, update on gemeinde_entry to deutschland_sammeln_bot;

CREATE TABLE gemeinde_result (
    id SERIAL PRIMARY KEY NOT NULL,
    gemeinde_name TEXT NOT NULL,
    occasion TEXT NOT NULL,
    UNIQUE(gemeinde_name, occasion),
    FOREIGN KEY (gemeinde_name, occasion) REFERENCES gemeinden(gemeinde_name, occasion)
);

ALTER TABLE gemeinde_result owner to admin;
grant insert, select, update on gemeinde_result to deutschland_sammeln_bot;


grant usage, select ON ALL SEQUENCES IN SCHEMA public TO deutschland_sammeln_bot;

