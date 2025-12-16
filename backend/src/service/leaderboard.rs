use std::collections::HashMap;

use thiserror::Error;

use crate::{
    client::AocClient,
    db::DbPool,
    model::{
        aoc::{AocPart, AocPuzzle},
        leaderboard::{AocMemberId, LeaderboardDto},
    },
    repository::LeaderboardRepository,
    service::aoc_utils::AocUtils,
};

pub struct LeaderboardService {}

#[derive(Error, Debug)]
pub enum LeaderboardError {
    #[error("Leaderboard not cached and no session token provided.")]
    NotCached,
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Failed to fetch leaderboard from AoC: {0}")]
    FetchError(#[from] reqwest::Error),
    #[error("Failed to parse leaderboard data: {0}")]
    ParseError(#[from] serde_json::Error),
}

impl From<rusqlite::Error> for LeaderboardError {
    fn from(err: rusqlite::Error) -> Self {
        LeaderboardError::DatabaseError(err.to_string())
    }
}

impl From<r2d2::Error> for LeaderboardError {
    fn from(err: r2d2::Error) -> Self {
        LeaderboardError::DatabaseError(err.to_string())
    }
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
        pool: &DbPool,
        year: u32,
        board_id: u32,
        session_token: Option<&str>,
    ) -> Result<LeaderboardDto, LeaderboardError> {
        let lbr = LeaderboardRepository::new();

        // Check cache (get connection, use it, release it before async work)
        let cached_result = {
            let conn = pool.get()?;
            lbr.get_leaderboard(&conn, year, board_id)
        };

        if let Some(cached) = cached_result {
            if (chrono::Utc::now() - cached.updated_at).num_seconds() < 900
                || session_token.is_none()
            {
                println!(
                    "Using cached leaderboard for year {}, board {}, age {} seconds",
                    year,
                    board_id,
                    (chrono::Utc::now() - cached.updated_at).num_seconds()
                );
                return Ok(cached);
            }
        }

        if session_token.is_none() {
            return Err(LeaderboardError::NotCached);
        }

        println!(
            "Fetching leaderboard for year {}, board {} from AoC",
            year, board_id
        );

        // Fetch from AoC API (async work without holding connection)
        let response = AocClient::new()
            .fetch_leaderboard(year, board_id, session_token.unwrap())
            .await
            .map_err(LeaderboardError::FetchError)?;

        // Save to database (get fresh connection)
        let data = serde_json::to_string(&response).map_err(LeaderboardError::ParseError)?;
        let conn = pool.get()?;
        lbr.save_leaderboard(&conn, year, board_id, &data)
            .map_err(Into::into)
    }

    pub async fn get_or_create_leaderboard_range(
        &self,
        pool: &DbPool,
        years: &[u32],
        board_id: u32,
        session_token: Option<&str>,
    ) -> Vec<Result<LeaderboardDto, LeaderboardError>> {
        let mut results = Vec::new();
        for &year in years {
            match self
                .get_or_create_leaderboard(pool, year, board_id, session_token)
                .await
            {
                Ok(leaderboard) => results.push(Ok(leaderboard)),
                Err(e) => results.push(Err(e)),
            }
        }
        results
    }

    pub async fn get_or_create_all_leaderboards(
        &self,
        pool: &DbPool,
        board_id: u32,
        session_token: Option<&str>,
    ) -> Vec<Result<LeaderboardDto, LeaderboardError>> {
        let years: Vec<u32> =
            (AocUtils::earliest_puzzle().year..=AocUtils::latest_puzzle().year).collect();
        self.get_or_create_leaderboard_range(pool, &years, board_id, session_token)
            .await
    }

    pub async fn get_bingo_options(
        &self,
        pool: &DbPool,
        years: Option<&[u32]>,
        board_id: u32,
        session_token: Option<&str>,
        member_ids: Option<&[AocMemberId]>,
        game_creation_date: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<Vec<AocPuzzle>, BingoError> {
        let years = match years {
            Some(y) => y.to_vec(),
            None => (AocUtils::earliest_puzzle().year..=AocUtils::latest_puzzle().year).collect(),
        };
        let leaderboards = self
            .get_or_create_leaderboard_range(pool, &years, board_id, session_token)
            .await
            .into_iter()
            .filter_map(|r| r.ok().map(|l| (l.year as u32, l)))
            .collect::<HashMap<_, _>>();

        let all_puzzles = AocUtils::puzzles_for_years(&years);

        let mut bingo_options = Vec::<AocPuzzle>::new();

        let solved_after_game_creation = |ts: u64| -> bool {
            ts >= game_creation_date
                .map(|d| d.timestamp() as u64)
                .unwrap_or(0)
        };

        for puzzle in all_puzzles {
            let year = puzzle.date.year;
            let day = puzzle.date.day;

            if let Some(leaderboard) = leaderboards.get(&year) {
                let member_data: Vec<_> = leaderboard
                    .data
                    .members
                    .values()
                    .filter(|m| match member_ids {
                        Some(ids) => ids.contains(&m.id),
                        None => true,
                    })
                    .collect();
                match puzzle.part {
                    AocPart::One => {
                        // If part one is requested, ensure nobody has solved it
                        let nobody_solved = member_data.iter().all(|member| {
                            member
                                .completion_day_level
                                .get(&day)
                                .map(|day_completion| {
                                    match day_completion.get(&AocPart::One.into()) {
                                        Some(t) => solved_after_game_creation(t.get_star_ts),
                                        None => true,
                                    }
                                })
                                .unwrap_or(true)
                        });
                        if !nobody_solved {
                            continue;
                        }
                    }
                    AocPart::Two => {
                        if day == AocUtils::get_calendar_size_of_year(year).unwrap() {
                            // Do not allow part two of the last day
                            continue;
                        }
                        // If part two is requested, ensure part one is solved for everyone
                        let nobody_solved_but_meeting_requirements =
                            member_data.iter().all(|member| {
                                member
                                    .completion_day_level
                                    .get(&day)
                                    .map(|day_completion| {
                                        day_completion.get(&AocPart::One.into()).is_some()
                                            && match day_completion.get(&AocPart::Two.into()) {
                                                Some(t) => {
                                                    solved_after_game_creation(t.get_star_ts)
                                                }
                                                None => true,
                                            }
                                    })
                                    .unwrap_or(false)
                            });
                        // Or nobody has solved at all
                        let nobody_solved_at_all = member_data.iter().all(|member| {
                            member
                                .completion_day_level
                                .get(&day)
                                .map(|day_completion| {
                                    (match day_completion.get(&AocPart::One.into()) {
                                        Some(t) => solved_after_game_creation(t.get_star_ts),
                                        None => true,
                                    }) && (match day_completion.get(&AocPart::Two.into()) {
                                        Some(t) => solved_after_game_creation(t.get_star_ts),
                                        None => true,
                                    })
                                })
                                .unwrap_or(true)
                        });
                        if !nobody_solved_but_meeting_requirements && !nobody_solved_at_all {
                            continue;
                        }
                    }
                }
                bingo_options.push(puzzle);
            } else {
                bingo_options.push(puzzle);
            }
        }
        if bingo_options.is_empty() {
            Err(BingoError::NoOptions)
        } else {
            Ok(bingo_options)
        }
    }
}
