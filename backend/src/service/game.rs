use rand::Rng;
use rand::distributions::Alphanumeric;
use thiserror::Error;

use crate::{
    db::{DbConnection, DbPool, with_transaction},
    model::{
        game::{GameDto, GameId, GameLeaderboardMemberDto, GameMembershipDto},
        leaderboard::{AocLeaderboardId, AocMemberId},
    },
    repository::GameRepository,
    service::LeaderboardService,
};

pub struct GameService {}

#[derive(Error, Debug)]
pub enum GameError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Game not found: {0}")]
    NotFound(GameId),
    #[error("Failed to generate unique game ID after {0} attempts")]
    IdGenerationFailed(u32),
    #[error("Leaderboard not found")]
    LeaderboardNotFound,
}

impl From<rusqlite::Error> for GameError {
    fn from(err: rusqlite::Error) -> Self {
        GameError::DatabaseError(err.to_string())
    }
}

impl From<r2d2::Error> for GameError {
    fn from(err: r2d2::Error) -> Self {
        GameError::DatabaseError(err.to_string())
    }
}

#[derive(Error, Debug)]
pub enum GameMembershipError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Membership not found: {0}")]
    NotFound(u32),
    #[error("Game not found: {0}")]
    GameNotFound(GameId),
}

impl From<rusqlite::Error> for GameMembershipError {
    fn from(err: rusqlite::Error) -> Self {
        GameMembershipError::DatabaseError(err.to_string())
    }
}

impl From<r2d2::Error> for GameMembershipError {
    fn from(err: r2d2::Error) -> Self {
        GameMembershipError::DatabaseError(err.to_string())
    }
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
        conn: &DbConnection,
        leaderboard_id: AocLeaderboardId,
        session_token: &str,
        max_attempts: u32,
    ) -> Result<GameDto, GameError> {
        let repo = GameRepository::new();

        for attempt in 0..max_attempts {
            let id = Self::generate_game_id();

            match repo.create_game(conn, &id, leaderboard_id, session_token) {
                Ok(game) => return Ok(game),
                Err(rusqlite::Error::SqliteFailure(err, _))
                    if err.code == rusqlite::ErrorCode::ConstraintViolation
                        && attempt < max_attempts - 1 =>
                {
                    // ID collision, retry with new ID
                    continue;
                }
                Err(e) => return Err(e.into()),
            }
        }

        Err(GameError::IdGenerationFailed(max_attempts))
    }

    /// Get a game by ID
    pub fn get_game(&self, conn: &DbConnection, id: &str) -> Result<GameDto, GameError> {
        let repo = GameRepository::new();

        repo.get_game(conn, id)
            .ok_or_else(|| GameError::NotFound(id.to_string()))
    }

    pub async fn get_possible_members(
        &self,
        pool: &DbPool,
        id: &str,
    ) -> Result<Vec<GameLeaderboardMemberDto>, GameError> {
        // Get game info (sync, release connection before async work)
        let game = {
            let conn = pool.get()?;
            self.get_game(&conn, id)?
        };

        // Fetch leaderboards (async)
        let lbs = LeaderboardService::new();
        let leaderboards = lbs
            .get_or_create_all_leaderboards(pool, game.leaderboard_id, Some(&game.session_token))
            .await;

        let leaderboard = leaderboards
            .into_iter()
            .find_map(|r| r.ok())
            .ok_or(GameError::LeaderboardNotFound)?;

        Ok(leaderboard
            .data
            .members
            .values()
            .map(|member| GameLeaderboardMemberDto {
                id: member.id,
                name: member.name.clone(),
            })
            .collect())
    }

    /// Get all games (optional - for debugging/admin)
    pub fn get_all_games(&self, conn: &DbConnection) -> Vec<GameDto> {
        let repo = GameRepository::new();
        repo.get_all_games(conn)
    }

    /// Create a membership in a game
    pub fn create_membership(
        &self,
        conn: &mut DbConnection,
        game_id: &str,
        member_id: AocMemberId,
        member_name: &str,
    ) -> Result<GameMembershipDto, GameMembershipError> {
        with_transaction(conn, |tx| {
            let repo = GameRepository::new();

            // Verify game exists first (within transaction)
            if repo.get_game(tx, game_id).is_none() {
                return Err(GameMembershipError::GameNotFound(game_id.to_string()));
            }

            repo.create_membership(tx, game_id, member_id, member_name)
                .map_err(Into::into)
        })
    }

    /// Delete a membership by its ID
    pub fn delete_membership(
        &self,
        conn: &DbConnection,
        membership_id: u32,
    ) -> Result<(), GameMembershipError> {
        let repo = GameRepository::new();
        repo.delete_membership(conn, membership_id)
            .map_err(Into::into)
    }

    /// Delete a membership by game_id and member_id
    pub fn delete_membership_by_game_and_member(
        &self,
        conn: &mut DbConnection,
        game_id: &str,
        member_id: AocMemberId,
    ) -> Result<(), GameMembershipError> {
        with_transaction(conn, |tx| {
            let repo = GameRepository::new();

            // Verify game exists first (within transaction)
            if repo.get_game(tx, game_id).is_none() {
                return Err(GameMembershipError::GameNotFound(game_id.to_string()));
            }

            repo.delete_membership_by_game_and_member(tx, game_id, member_id)
                .map_err(Into::into)
        })
    }

    /// Get all memberships for a game
    pub fn get_memberships(
        &self,
        conn: &mut DbConnection,
        game_id: &str,
    ) -> Result<Vec<GameMembershipDto>, GameError> {
        with_transaction(conn, |tx| {
            let repo = GameRepository::new();

            // Verify game exists (within transaction)
            if repo.get_game(tx, game_id).is_none() {
                return Err(GameError::NotFound(game_id.to_string()));
            }

            Ok(repo.get_memberships_by_game(tx, game_id))
        })
    }
}
