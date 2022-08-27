CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE IF NOT EXISTS webhooks (
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4 (),
    url char(256) NOT NULL,
    last_sent timestamp,
    event char(256) NOT NULL
);
