CREATE TABLE weather (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4 () NOT NULL,
    "location" point NOT NULL,
    "timestamp" timestamp NOT NULL
);