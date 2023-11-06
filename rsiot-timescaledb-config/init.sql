CREATE DATABASE db_conf;
CREATE DATABASE db_data;

\c db_data
CREATE EXTENSION IF NOT EXISTS timescaledb;

-- enum agg_type
CREATE TYPE agg_type AS ENUM (
    'curr',
    'first',
    'inc',
    'sum',
    'mean',
    'min',
    'max'
);

-- table raw
CREATE TABLE raw (
    ts          TIMESTAMPTZ         NOT NULL,
    entity      TEXT                NOT NULL,
    attr        TEXT                NOT NULL,
    value       DOUBLE PRECISION    NULL,
    agg         AGG_TYPE            NOT NULL,
    aggts       TIMESTAMPTZ         NULL,
    aggnext     AGG_TYPE[]          NULL,
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
SELECT add_compression_policy('raw', INTERVAL '100000 hours');

-- agg_30min
CREATE TABLE agg_30min (LIKE raw);


-- create databases for test
CREATE DATABASE db_data_test WITH TEMPLATE db_data;
CREATE DATABASE db_conf_test WITH TEMPLATE db_conf;