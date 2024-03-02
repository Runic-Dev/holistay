-- Add up migration script here
CREATE TABLE IF NOT EXISTS settings (
  setting TEXT UNIQUE NOT NULL,
  value TEXT 
);
