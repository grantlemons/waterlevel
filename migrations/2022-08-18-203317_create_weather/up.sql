CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE weather (
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4 (),
    "location" POINT NOT NULL,
    "timestamp" TIMESTAMP NOT NULL
);