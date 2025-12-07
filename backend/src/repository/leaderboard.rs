use chrono::DateTime;
use sqlite::{Connection, Row, Value};

use crate::model::leaderboard::LeaderboardDto;

pub struct LeaderboardRepository;

impl TryFrom<Row> for LeaderboardDto {
    type Error = sqlite::Error;

    fn try_from(row: Row) -> Result<Self, Self::Error> {
        let id: i64 = row.try_read("id")?;
        let year: i64 = row.try_read("year")?;
        let board_id: i64 = row.try_read("leaderboard_id")?;
        let data = row.try_read::<&str, _>("data")?.to_owned();
        let created_at = row.try_read::<i64, _>("created_at")?.to_owned();
        let updated_at = row.try_read::<i64, _>("updated_at")?.to_owned();

        Ok(LeaderboardDto {
            id,
            year,
            board_id,
            data,
            created_at: DateTime::from_timestamp_secs(created_at).unwrap(),
            updated_at: DateTime::from_timestamp_secs(updated_at).unwrap(),
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
    ) -> Result<LeaderboardDto, sqlite::Error> {
        let mut statement = conn.prepare(
            "INSERT INTO leaderboard_cache (year, leaderboard_id, data)
             VALUES (:year, :board_id, :data)
             ON CONFLICT(year, leaderboard_id) DO UPDATE SET
             data = :data,
             updated_at = :updated_at
             RETURNING *;",
        )?;
        statement.bind::<&[(_, Value)]>(&[
            (":year", (year as i64).into()),
            (":board_id", (board_id as i64).into()),
            (":data", data.into()),
        ])?;

        match self.consume_statement_as_leaderboard(&mut statement) {
            Ok(Some(dto)) => Ok(dto),
            Ok(None) => panic!("Expected leaderboard to be returned after insert/update."),
            Err(e) => Err(e),
        }
    }

    pub fn get_leaderboard(
        &self,
        conn: &Connection,
        year: u32,
        board_id: u32,
    ) -> Option<LeaderboardDto> {
        let mut statement = conn
            .prepare("SELECT * FROM leaderboard_cache WHERE year = :year AND leaderboard_id = :board_id;")
            .unwrap();
        statement
            .bind::<&[(_, Value)]>(&[
                (":year", (year as i64).into()),
                (":board_id", (board_id as i64).into()),
            ])
            .unwrap();
        self.consume_statement_as_leaderboard(&mut statement)
            .ok()
            .flatten()
    }

    pub fn get_all_leaderboard_by_id(
        &self,
        conn: &Connection,
        board_id: u32,
    ) -> Vec<LeaderboardDto> {
        let mut statement = conn
            .prepare("SELECT * FROM leaderboard_cache WHERE leaderboard_id = :board_id ORDER BY year ASC;")
            .unwrap();
        statement
            .bind::<&[(_, Value)]>(&[(":board_id", (board_id as i64).into())])
            .unwrap();

        let mut leaderboards = Vec::new();
        for row_result in statement.iter() {
            match row_result {
                Ok(row) => match row.try_into() {
                    Ok(dto) => leaderboards.push(dto),
                    Err(e) => {
                        eprintln!(
                            "Failed to convert row to LeaderboardDto for board_id={}: {:?}",
                            board_id, e
                        );
                    }
                },
                Err(e) => {
                    eprintln!(
                        "Database error fetching leaderboard for board_id={}: {:?}",
                        board_id, e
                    );
                }
            }
        }
        leaderboards
    }

    fn consume_statement(&self, statement: &mut sqlite::Statement) -> Result<(), sqlite::Error> {
        while let Ok(state) = statement.next() {
            if state == sqlite::State::Done {
                break;
            }
        }
        Ok(())
    }

    fn consume_statement_as_leaderboard(
        &self,
        statement: &mut sqlite::Statement,
    ) -> Result<Option<LeaderboardDto>, sqlite::Error> {
        let result = match statement.iter().next() {
            None => Ok(None),
            Some(row) => match row {
                Ok(row) => match row.try_into() {
                    Ok(dto) => Ok(Some(dto)),
                    Err(e) => {
                        eprintln!("Failed to convert row to LeaderboardDto: {:?}", e);
                        Err(e)
                    }
                },
                Err(e) => {
                    eprintln!("Database error fetching leaderboard: {:?}", e);
                    Err(e)
                }
            },
        };
        self.consume_statement(statement)?;
        result
    }
}
