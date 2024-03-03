-- Add up migration script here
ALTER TABLE room_group
  ADD COLUMN description TEXT;
