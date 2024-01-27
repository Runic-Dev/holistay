-- Add up migration script here
CREATE TABLE IF NOT EXISTS room_group (
  id TEXT UNIQUE NOT NULL,
  property_id TEXT NOT NULL,
  name TEXT NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
