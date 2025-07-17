-- Users table
CREATE TABLE IF NOT EXISTS users (
  id TEXT NOT NULL PRIMARY KEY,
  created_date TEXT NOT NULL DEFAULT (CURRENT_TIMESTAMP),
  modified_date TEXT NOT NULL DEFAULT (CURRENT_TIMESTAMP),
  deleted_date TEXT,
  last_login_date TEXT,
  tz TEXT NOT NULL DEFAULT 'UTC',
  email TEXT NOT NULL UNIQUE,
  backup_email TEXT UNIQUE
);

-- Webauthn credentials (passkeys)
CREATE TABLE credentials (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  user_id TEXT NOT NULL,
  credential_id TEXT NOT NULL UNIQUE,
  public_key TEXT,
  attestation_type TEXT NOT NULL,
  aaguid TEXT DEFAULT '00000000-0000-0000-0000-000000000000',
  signature_count INTEGER NOT NULL DEFAULT 0,
  created_date TEXT DEFAULT (datetime('now')),
  modified_date TEXT DEFAULT (datetime('now')),
  last_used_date TEXT,
  type TEXT,
  transports TEXT,
  backup_eligible INTEGER DEFAULT 0,
  backup_state INTEGER DEFAULT 0,
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Profile
CREATE TABLE IF NOT EXISTS profiles (
  id TEXT NOT NULL PRIMARY KEY,
  created_date TEXT NOT NULL DEFAULT (CURRENT_TIMESTAMP),
  modified_date TEXT NOT NULL DEFAULT (CURRENT_TIMESTAMP),
  deleted_date TEXT,
  display_name TEXT,
  user_id TEXT NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

-- For fast joins
CREATE INDEX IF NOT EXISTS idx_credentials_user_id ON credentials(user_id);
CREATE UNIQUE INDEX IF NOT EXISTS idx_profiles_user_id ON profiles(user_id);
