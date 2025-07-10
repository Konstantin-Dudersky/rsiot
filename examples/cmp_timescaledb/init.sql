CREATE DATABASE db_conf;
CREATE DATABASE db_data;

\c db_data
CREATE EXTENSION IF NOT EXISTS timescaledb;

-- enum agg_type
CREATE TYPE AggType AS ENUM (
    'Curr',
    'First',
    'Inc',
    'Sum',
    'Mean',
    'Min',
    'Max',
    'Count'
);

-- table raw
CREATE TABLE raw (
    ts          TIMESTAMPTZ         NOT NULL,
    entity      TEXT                NOT NULL,
    attr        TEXT                NOT NULL,
    value       DOUBLE PRECISION    NULL,
    agg         AggType             NOT NULL,
    aggts       TIMESTAMPTZ         NULL,
    aggnext     AggType[]           NULL,
    UNIQUE (ts, entity, attr, agg)
);
SELECT create_hypertable(
    'raw', 'ts',
    chunk_time_interval => INTERVAL '24 hours'
);
ALTER TABLE raw SET (
    timescaledb.compress,
    timescaledb.compress_segmentby='entity, attr, agg'
);
SELECT add_compression_policy('raw', INTERVAL '24 hours');

-- agg_30min
CREATE TABLE agg_30min (LIKE raw);


-- create databases for test
CREATE DATABASE db_data_test WITH TEMPLATE db_data;
CREATE DATABASE db_conf_test WITH TEMPLATE db_conf;
