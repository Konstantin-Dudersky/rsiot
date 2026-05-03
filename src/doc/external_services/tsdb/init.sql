CREATE DATABASE db_data;

\c db_data
CREATE EXTENSION IF NOT EXISTS timescaledb;

-- -- table raw
-- CREATE TABLE raw (
--     time        TIMESTAMPTZ         NOT NULL,
--     prj         TEXT                NOT NULL,
--     hst         TEXT                NOT NULL,
--     svc         TEXT                NOT NULL,
--     cmp         TEXT                NOT NULL,
--     key         TEXT                NOT NULL,
--     value       DOUBLE PRECISION    NULL,
--     UNIQUE (time, prj, hst, svc, cmp, key)
-- ) WITH (
--    tsdb.hypertable,
--    tsdb.partition_column = 'time',
--    tsdb.chunk_interval = 'PT1M',
--    tsdb.segmentby = 'prj, hst, svc, cmp, key',
--    tsdb.orderby = 'time ASC'
-- );

-- CALL remove_columnstore_policy('raw');
-- CALL add_columnstore_policy('raw', after => INTERVAL 'PT10M');
