CREATE TABLE contacts (
  id bigserial primary key,
  name VARCHAR,
  email VARCHAR,
  created_at TIMESTAMPTZ NOT NULL default now(),
  updated_at TIMESTAMPTZ NOT NULL default now()
);
