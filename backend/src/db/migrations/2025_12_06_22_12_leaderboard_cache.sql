CREATE TABLE IF NOT EXISTS leaderboard_cache (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    leaderboard_id INTEGER NOT NULL,
    year INTEGER NOT NULL,
    data TEXT NOT NULL,
    created_at INTEGER DEFAULT (unixepoch()),
    updated_at INTEGER DEFAULT (unixepoch())
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_leaderboard_cache_unique
ON leaderboard_cache (leaderboard_id, year);

CREATE TRIGGER IF NOT EXISTS trg_leaderboard_cache_updated_at
AFTER UPDATE ON leaderboard_cache
FOR EACH ROW
BEGIN
    UPDATE leaderboard_cache SET updated_at = unixepoch() WHERE id = OLD.id;
END;
