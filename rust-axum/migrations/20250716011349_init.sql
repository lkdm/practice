-- Users are internal representations of a person
CREATE TABLE IF NOT EXISTS user (
  id TEXT NOT NULL PRIMARY KEY,
  created_date TEXT NOT NULL DEFAULT (CURRENT_TIMESTAMP),
  modified_date TEXT NOT NULL DEFAULT (CURRENT_TIMESTAMP),
  deleted_date TEXT,
  last_login_date TEXT,
  tz TEXT NOT NULL DEFAULT 'UTC',
  email TEXT NOT NULL UNIQUE,
  backup_email TEXT UNIQUE
);

-- Profile is a public representation of a person
CREATE TABLE IF NOT EXISTS profile (
  id TEXT NOT NULL PRIMARY KEY,
  created_date TEXT NOT NULL DEFAULT (CURRENT_TIMESTAMP),
  modified_date TEXT NOT NULL DEFAULT (CURRENT_TIMESTAMP),
  deleted_date TEXT,

  display_name TEXT NOT NULL,

  user_id TEXT,
  FOREIGN KEY (user_id) REFERENCES user (id) ON DELETE CASCADE
);

