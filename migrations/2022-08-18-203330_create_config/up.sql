CREATE TABLE config (
    key char(256) PRIMARY KEY NOT NULL,
    value char(256) NOT NULL,
    "timestamp" timestamp NOT NULL
);
