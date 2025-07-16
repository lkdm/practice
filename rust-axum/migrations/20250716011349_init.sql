-- Allow triggers
PRAGMA recursive_triggers = ON;

-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
  id TEXT PRIMARY KEY,
  created_date TEXT NOT NULL DEFAULT (CURRENT_TIMESTAMP),
  modified_date TEXT NOT NULL DEFAULT (CURRENT_TIMESTAMP),
  deleted_date TEXT
);

-- Update modified_date
CREATE TRIGGER IF NOT EXISTS update_modified_date
AFTER UPDATE ON users
FOR EACH ROW
BEGIN
  UPDATE users
  SET modified_date = CURRENT_TIMESTAMP
  WHERE id = OLD.id;
END;
