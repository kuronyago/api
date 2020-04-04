-- Your SQL goes here
CREATE TABLE projects (
  id SERIAL PRIMARY KEY,
  external uuid UNIQUE NOT NULL,
  creator uuid NOT NULL,
  free int NOT NULL,
  spent int NOT NULL,
  frozen int NOT NULL,
  created_at timestamptz NOT NULL,
  updated_at timestamptz NOT NULL
)