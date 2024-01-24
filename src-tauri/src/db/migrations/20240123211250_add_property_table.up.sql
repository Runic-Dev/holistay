-- Add up migration script here
CREATE TABLE IF NOT EXISTS property (
  id TEXT UNIQUE NOT NULL,
  name TEXT UNIQUE NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- CREATE TRIGGER update_property_updated_at
-- AFTER INSERT ON property
-- FOR EACH ROW
-- BEGIN
--     UPDATE property SET created_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
-- END;
