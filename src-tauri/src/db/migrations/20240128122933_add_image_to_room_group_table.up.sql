-- Add up migration script here
ALTER TABLE room_group
  ADD COLUMN image TEXT;
