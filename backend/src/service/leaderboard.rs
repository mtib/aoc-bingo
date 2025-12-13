use std::collections::HashMap;

use thiserror::Error;

use crate::{
    client::AocClient,
    db::{DatabaseManager, get_db},
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
        years: Option<&[u32]>,
        board_id: u32,
        session_token: Option<&str>,
        member_ids: Option<&[AocMemberId]>,
    ) -> Result<Vec<AocPuzzle>, BingoError> {
        let years = match years {
            Some(y) => y.to_vec(),
            None => (AocUtils::earliest_puzzle().year..=AocUtils::latest_puzzle().year).collect(),
        };
        let leaderboards = self
            .get_or_create_leaderboard_range(&years, board_id, session_token)
            .await
            .into_iter()
            .filter_map(|r| r.ok().map(|l| (l.year as u32, l)))
            .collect::<HashMap<_, _>>();

        let all_puzzles = AocUtils::puzzles_for_years(&years);

        let mut bingo_options = Vec::<AocPuzzle>::new();

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
                                    day_completion.get(&AocPart::One.into()).is_none()
                                })
                                .unwrap_or(true)
                        });
                        if !nobody_solved {
                            continue;
                        }
                    }
                    AocPart::Two => {
                        // If part two is requested, ensure part one is solved for everyone
                        let nobody_solved_but_meeting_requirements =
                            member_data.iter().all(|member| {
                                member
                                    .completion_day_level
                                    .get(&day)
                                    .map(|day_completion| {
                                        day_completion.get(&AocPart::One.into()).is_some()
                                            && day_completion.get(&AocPart::Two.into()).is_none()
                                    })
                                    .unwrap_or(false)
                            });
                        // Or nobody has solved at all
                        let nobody_solved_at_all = member_data.iter().all(|member| {
                            member
                                .completion_day_level
                                .get(&day)
                                .map(|day_completion| {
                                    day_completion.get(&AocPart::One.into()).is_none()
                                        && day_completion.get(&AocPart::Two.into()).is_none()
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
