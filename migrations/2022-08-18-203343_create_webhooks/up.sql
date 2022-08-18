CREATE TABLE webhooks (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4 () NOT NULL,
    url char(256) NOT NULL,
    last_sent timestamp,
    event char(256) NOT NULL,
);