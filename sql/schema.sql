DROP SCHEMA IF EXISTS testing CASCADE;
CREATE SCHEMA testing;

CREATE TABLE testing.users (
    id          BIGSERIAL PRIMARY KEY,
    email       VARCHAR(200) NOT NULL,
    first_name  VARCHAR(200) NOT NULL,
    last_name   VARCHAR(200) NOT NULL,
    username    VARCHAR(50) UNIQUE NOT NULL,
    UNIQUE (username)
);

CREATE TABLE testing.plantdata (
    id              BIGSERIAL PRIMARY KEY,
    plant_id        INTEGER NOT NULL,
    created_at      INTEGER NOT NULL,
    updated_at      INTEGER NOT NULL,
    planned_data    INTEGER NOT NULL,
    unplanned_data  INTEGER NOT NULL
);
