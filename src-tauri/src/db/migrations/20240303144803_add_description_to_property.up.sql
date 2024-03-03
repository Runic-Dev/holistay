-- Add up migration script here
ALTER TABLE property
  ADD COLUMN description TEXT;
