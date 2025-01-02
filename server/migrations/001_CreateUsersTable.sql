CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS users (
    id              UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    username        VARCHAR NOT NULL,
    email           VARCHAR NOT NULL unique,
    password_salt   VARCHAR NOT NULL,
    password_hash   VARCHAR NOT NULL,
    created_at      timestamp NOT NULL DEFAULT current_timestamp,
    updated_at      timestamp NOT NULL DEFAULT current_timestamp
);