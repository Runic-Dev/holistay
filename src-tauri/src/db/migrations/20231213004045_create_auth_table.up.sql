-- Add up migration script here
CREATE TABLE IF NOT EXISTS auth (
  id TEXT NOT NULL,
  username TEXT NOT NULL,
  password TEXT NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_auth_updated_at
AFTER UPDATE ON auth
FOR EACH ROW
BEGIN
    UPDATE auth SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
END;
