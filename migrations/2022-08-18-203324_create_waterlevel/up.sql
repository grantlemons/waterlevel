CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE water_levels (
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4 (),
    "location" POINT NOT NULL,
    "timestamp" TIMESTAMP NOT NULL,
    weather_id UUID,
    level DOUBLE PRECISION NOT NULL
);
