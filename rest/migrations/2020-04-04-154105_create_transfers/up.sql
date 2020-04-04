-- Your SQL goes here
CREATE TABLE transfers (
  id SERIAL PRIMARY KEY,
  external uuid UNIQUE NOT NULL,
  sender uuid NOT NULL,
  recipient uuid NOT NULL,
  issued int NOT NULL,
  gained int NOT NULL,
  created_at timestamptz NOT NULL,
  updated_at timestamptz NOT NULL
)