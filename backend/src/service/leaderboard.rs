use thiserror::Error;

use crate::{
    client::AocClient, db::DatabaseManager, model::leaderboard::LeaderboardDto,
    repository::LeaderboardRepository,
};

pub struct LeaderboardService {}

#[derive(Error, Debug)]
pub enum LeaderboardError {
    #[error("Leaderboard not cached and no session token provided.")]
    NotCached,
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlite::Error),
    #[error("Failed to fetch leaderboard from AoC: {0}")]
    FetchError(#[from] reqwest::Error),
    #[error("Failed to parse leaderboard data: {0}")]
    ParseError(#[from] serde_json::Error),
}

impl LeaderboardService {
    pub fn new() -> Self {
        LeaderboardService {}
    }

    /// Returns error if leaderboard is not cached and [session_token] is None
    pub async fn get_or_create_leaderboard(
        &self,
        year: u32,
        board_id: u32,
        session_token: Option<&str>,
    ) -> Result<LeaderboardDto, LeaderboardError> {
        let lbr = LeaderboardRepository::new();
        {
            let dbm = DatabaseManager::new();
            let conn = dbm.get_connection();
            if let Some(cached) = lbr.get_leaderboard(conn, year, board_id) {
                return Ok(cached);
            }

            if session_token.is_none() {
                return Err(LeaderboardError::NotCached);
            }
        }

        AocClient::new()
            .fetch_leaderboard(year, board_id, session_token.unwrap())
            .await
            .map_err(LeaderboardError::FetchError)
            .and_then(|response| {
                let dbm = DatabaseManager::new();
                let data =
                    serde_json::to_string(&response).map_err(LeaderboardError::ParseError)?;
                lbr.save_leaderboard(dbm.get_connection(), year, board_id, &data)
                    .map_err(LeaderboardError::DatabaseError)
            })
    }
}
