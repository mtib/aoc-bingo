-- Game table: stores game instances with 8-char alphanumeric IDs
CREATE TABLE IF NOT EXISTS games (
    id TEXT PRIMARY KEY CHECK(length(id) = 8),
    leaderboard_id INTEGER NOT NULL,
    session_token TEXT NOT NULL,
    created_at INTEGER DEFAULT (unixepoch()),
    updated_at INTEGER DEFAULT (unixepoch())
);

CREATE INDEX IF NOT EXISTS idx_games_leaderboard_id ON games(leaderboard_id);

CREATE TRIGGER IF NOT EXISTS trg_games_updated_at
AFTER UPDATE ON games
FOR EACH ROW
BEGIN
    UPDATE games SET updated_at = unixepoch() WHERE id = OLD.id;
END;

-- Game membership table: tracks members in each game
CREATE TABLE IF NOT EXISTS game_memberships (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    game_id TEXT NOT NULL,
    member_id INTEGER NOT NULL,
    member_name TEXT NOT NULL,
    created_at INTEGER DEFAULT (unixepoch()),
    FOREIGN KEY (game_id) REFERENCES games(id) ON DELETE CASCADE
);

-- Index for efficient lookups by game_id
CREATE INDEX IF NOT EXISTS idx_game_memberships_game_id ON game_memberships(game_id);
