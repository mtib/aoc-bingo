use std::collections::HashMap;

use thiserror::Error;

use crate::{
    client::AocClient,
    db::{DatabaseManager, get_db},
    model::{aoc::AocPuzzle, leaderboard::LeaderboardDto},
    repository::LeaderboardRepository,
    service::aoc_utils::AocUtils,
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

#[derive(Error, Debug)]
pub enum BingoError {
    #[error("No valid bingo options available.")]
    NoOptions,
    #[error("Leaderboard error: {0}")]
    LeaderboardError(#[from] LeaderboardError),
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
            let conn = get_db().unwrap();
            if let Some(cached) = lbr.get_leaderboard(&conn, year, board_id) {
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

    pub async fn get_or_create_leaderboard_range(
        &self,
        years: &[u32],
        board_id: u32,
        session_token: Option<&str>,
    ) -> Vec<Result<LeaderboardDto, LeaderboardError>> {
        let mut results = Vec::new();
        for &year in years {
            match self
                .get_or_create_leaderboard(year, board_id, session_token)
                .await
            {
                Ok(leaderboard) => results.push(Ok(leaderboard)),
                Err(e) => results.push(Err(e)),
            }
        }
        results
    }

    pub async fn get_bingo_options(
        &self,
        years: &[u32],
        board_id: u32,
        session_token: Option<&str>,
    ) -> Result<Vec<AocPuzzle>, BingoError> {
        let leaderboards = self
            .get_or_create_leaderboard_range(years, board_id, session_token)
            .await
            .into_iter()
            .filter_map(|r| r.ok().map(|l| (l.year, l)))
            .collect::<HashMap<i64, LeaderboardDto>>();

        let all_puzzles = AocUtils::puzzles_for_years(years);

        let mut bingo_options = Vec::<AocPuzzle>::new();

        for puzzle in all_puzzles {
            let year = puzzle.date.year;
            let day = puzzle.date.day;

            todo!()
        }
        todo!()
    }
}
