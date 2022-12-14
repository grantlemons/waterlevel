CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE IF NOT EXISTS weather (
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4 (),
    "location" POINT NOT NULL,
    "timestamp" TIMESTAMP NOT NULL,
    temp float8 NOT NULL,
    temp_min float8 NOT NULL,
    temp_max float8 NOT NULL,
    pressure float8 NOT NULL,
    humidity smallint NOT NULL,
    weather_id smallint NOT NULL,
    weather_name char(256) NOT NULL
);
