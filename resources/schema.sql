CREATE EXTENSION IF NOT EXISTS timescaledb CASCADE;

CREATE TYPE entry_type AS ENUM ('sgv', 'mbg', 'cal', 'etc');

CREATE TABLE entries (
  date TIMESTAMPTZ NOT NULL,
  entry_type entry_type NOT NULL,
  sgv DOUBLE PRECISION NULL,
  direction VARCHAR(4) NULL,
  noise DOUBLE PRECISION NULL,
  filtered DOUBLE PRECISION NULL,
  unfiltered DOUBLE PRECISION NULL,
  rssi DOUBLE PRECISION NULL
);

SELECT create_hypertable('entries', 'date');

CREATE USER grafana WITH PASSWORD 'grafana';
GRANT USAGE ON SCHEMA public TO grafana;
GRANT SELECT ON public.entries TO grafana;
