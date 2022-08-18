CREATE TABLE water_levels (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4 () NOT NULL,
    "location" point NOT NULL,
    "timestamp" timestamp NOT NULL,
    weather_id DEFAULT uuid_generate_v4 (),
    level float NOT NULL
);