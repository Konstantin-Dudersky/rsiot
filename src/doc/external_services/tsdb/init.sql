CREATE DATABASE db_data;

\c db_data
CREATE EXTENSION IF NOT EXISTS timescaledb;

-- table raw
CREATE TABLE raw (
    time        TIMESTAMPTZ         NOT NULL,
    hst         TEXT                NOT NULL,
    svc         TEXT                NOT NULL,
    cmp         TEXT                NOT NULL,
    key         TEXT                NOT NULL,
    value       DOUBLE PRECISION    NULL,
    UNIQUE (time, hst, svc, cmp, key)
) WITH (
   tsdb.hypertable,
   tsdb.partition_column = 'time',
   tsdb.chunk_interval = 'PT10M',
   tsdb.segmentby = 'hst, svc, cmp, key',
   tsdb.orderby = 'time ASC'
);

CALL add_columnstore_policy('raw', after => INTERVAL 'PT10M');

-- agg_30min
CREATE TABLE agg_30min (LIKE raw);
