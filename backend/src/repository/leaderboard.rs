use chrono::DateTime;
use rusqlite::{Connection, Row, params};

use crate::model::leaderboard::{AocLeaderboardId, AocLeaderboardYearId, LeaderboardDto, Year};

pub struct LeaderboardRepository;

impl TryFrom<&Row<'_>> for LeaderboardDto {
    type Error = rusqlite::Error;

    fn try_from(row: &Row) -> Result<Self, Self::Error> {
        let id: i64 = row.get("id")?;
        let year: i64 = row.get("year")?;
        let board_id: i64 = row.get("leaderboard_id")?;
        let data: String = row.get("data")?;
        let created_at: i64 = row.get("created_at")?;
        let updated_at: i64 = row.get("updated_at")?;

        Ok(LeaderboardDto {
            id: id as AocLeaderboardYearId,
            year: year as Year,
            board_id: board_id as AocLeaderboardId,
            data: serde_json::from_str(&data).unwrap(),
            created_at: DateTime::from_timestamp(created_at, 0).unwrap(),
            updated_at: DateTime::from_timestamp(updated_at, 0).unwrap(),
        })
    }
}

impl LeaderboardRepository {
    pub fn new() -> Self {
        LeaderboardRepository
    }

    pub fn save_leaderboard(
        &self,
        conn: &Connection,
        year: u32,
        board_id: u32,
        data: &str,
    ) -> Result<LeaderboardDto, rusqlite::Error> {
        let mut statement = conn.prepare(
            "INSERT INTO leaderboard_cache (year, leaderboard_id, data)
             VALUES (?1, ?2, ?3)
             ON CONFLICT(year, leaderboard_id) DO UPDATE SET
             data = ?3
             RETURNING *;",
        )?;

        let mut rows = statement.query(params![year as i64, board_id as i64, data])?;

        if let Some(row) = rows.next()? {
            LeaderboardDto::try_from(row)
        } else {
            panic!("Expected leaderboard to be returned after insert/update.")
        }
    }

    pub fn get_leaderboard(
        &self,
        conn: &Connection,
        year: u32,
        board_id: u32,
    ) -> Option<LeaderboardDto> {
        let mut statement = conn
            .prepare("SELECT * FROM leaderboard_cache WHERE year = ?1 AND leaderboard_id = ?2;")
            .ok()?;
        let mut rows = statement
            .query(params![year as i64, board_id as i64])
            .ok()?;

        rows.next()
            .ok()?
            .and_then(|row| LeaderboardDto::try_from(row).ok())
    }

    pub fn get_all_leaderboard_by_id(
        &self,
        conn: &Connection,
        board_id: u32,
    ) -> Vec<LeaderboardDto> {
        let mut statement = conn
            .prepare("SELECT * FROM leaderboard_cache WHERE leaderboard_id = ?1 ORDER BY year ASC;")
            .unwrap();
        let rows = statement
            .query_map(params![board_id as i64], |row| {
                LeaderboardDto::try_from(row)
            })
            .unwrap();

        let mut leaderboards = Vec::new();
        for row_result in rows {
            match row_result {
                Ok(dto) => leaderboards.push(dto),
                Err(e) => {
                    eprintln!(
                        "Failed to convert row to LeaderboardDto for board_id={}: {:?}",
                        board_id, e
                    );
                }
            }
        }
        leaderboards
    }
}
