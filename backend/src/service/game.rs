use thiserror::Error;
use rand::Rng;
use rand::distributions::Alphanumeric;

use crate::{
    db::{DatabaseManager, get_db},
    model::game::{GameDto, GameId, GameMembershipDto},
    model::leaderboard::{AocLeaderboardId, AocMemberId},
    repository::GameRepository,
};

pub struct GameService {}

#[derive(Error, Debug)]
pub enum GameError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlite::Error),
    #[error("Game not found: {0}")]
    NotFound(GameId),
    #[error("Failed to generate unique game ID after {0} attempts")]
    IdGenerationFailed(u32),
}

#[derive(Error, Debug)]
pub enum GameMembershipError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlite::Error),
    #[error("Membership not found: {0}")]
    NotFound(u32),
    #[error("Game not found: {0}")]
    GameNotFound(GameId),
}

impl GameService {
    pub fn new() -> Self {
        GameService {}
    }

    /// Generate a random 8-character alphanumeric ID (lowercase)
    fn generate_game_id() -> GameId {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(|b| (b as char).to_ascii_lowercase())
            .collect()
    }

    /// Create a new game with a randomly generated ID
    /// Retries up to max_attempts times if there's an ID collision
    pub fn create_game(
        &self,
        leaderboard_id: AocLeaderboardId,
        session_token: &str,
        max_attempts: u32,
    ) -> Result<GameDto, GameError> {
        let repo = GameRepository::new();
        let dbm = DatabaseManager::new();
        let conn = dbm.get_connection();

        for attempt in 0..max_attempts {
            let id = Self::generate_game_id();

            match repo.create_game(conn, &id, leaderboard_id, session_token) {
                Ok(game) => return Ok(game),
                Err(sqlite::Error {
                    code: Some(19), // SQLITE_CONSTRAINT
                    ..
                }) if attempt < max_attempts - 1 => {
                    // ID collision, retry with new ID
                    continue;
                }
                Err(e) => return Err(GameError::DatabaseError(e)),
            }
        }

        Err(GameError::IdGenerationFailed(max_attempts))
    }

    /// Get a game by ID
    pub fn get_game(&self, id: &str) -> Result<GameDto, GameError> {
        let repo = GameRepository::new();
        let conn = get_db().unwrap();

        repo.get_game(&conn, id)
            .ok_or_else(|| GameError::NotFound(id.to_string()))
    }

    /// Get all games (optional - for debugging/admin)
    pub fn get_all_games(&self) -> Vec<GameDto> {
        let repo = GameRepository::new();
        let conn = get_db().unwrap();
        repo.get_all_games(&conn)
    }

    /// Create a membership in a game
    pub fn create_membership(
        &self,
        game_id: &str,
        member_id: AocMemberId,
        member_name: &str,
    ) -> Result<GameMembershipDto, GameMembershipError> {
        let repo = GameRepository::new();
        let dbm = DatabaseManager::new();
        let conn = dbm.get_connection();

        // Verify game exists first
        if repo.get_game(conn, game_id).is_none() {
            return Err(GameMembershipError::GameNotFound(game_id.to_string()));
        }

        repo.create_membership(conn, game_id, member_id, member_name)
            .map_err(GameMembershipError::DatabaseError)
    }

    /// Delete a membership by its ID
    pub fn delete_membership(&self, membership_id: u32) -> Result<(), GameMembershipError> {
        let repo = GameRepository::new();
        let dbm = DatabaseManager::new();
        let conn = dbm.get_connection();

        repo.delete_membership(conn, membership_id)
            .map_err(GameMembershipError::DatabaseError)
    }

    /// Get all memberships for a game
    pub fn get_memberships(&self, game_id: &str) -> Result<Vec<GameMembershipDto>, GameError> {
        let repo = GameRepository::new();
        let conn = get_db().unwrap();

        // Verify game exists
        if repo.get_game(&conn, game_id).is_none() {
            return Err(GameError::NotFound(game_id.to_string()));
        }

        Ok(repo.get_memberships_by_game(&conn, game_id))
    }
}
