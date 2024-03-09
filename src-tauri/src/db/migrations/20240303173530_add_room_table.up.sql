-- Add up migration script here
CREATE TABLE IF NOT EXISTS room (
  id TEXT UNIQUE NOT NULL,
  room_group_id TEXT NOT NULL,
  name TEXT NOT NULL,
  image TEXT,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
