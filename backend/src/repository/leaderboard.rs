use chrono::{DateTime, Utc};
use sqlite::{Connection, Value};

pub struct LeaderboardRepository;

pub struct LeaderboardDto {
    pub id: i64,
    pub year: i64,
    pub board_id: i64,
    pub data: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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
    ) -> Result<(), sqlite::Error> {
        let mut statement = conn.prepare(
            "INSERT INTO leaderboard_cache (year, leaderboard_id, data)
             VALUES (:year, :board_id, :data)
             ON CONFLICT(year, leaderboard_id) DO UPDATE SET
             data = :data,
             updated_at = :updated_at;",
        )?;
        statement.bind::<&[(_, Value)]>(&[
            (":year", (year as i64).into()),
            (":board_id", (board_id as i64).into()),
            (":data", data.into()),
        ])?;
        while let Ok(state) = statement.next() {
            if state == sqlite::State::Done {
                break;
            }
        }
        Ok(())
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
        if let Some(Ok(row)) = statement.iter().next() {
            let id: i64 = row.read("id");
            let year: i64 = row.read("year");
            let board_id: i64 = row.read("leaderboard_id");
            let data = row.read::<&str, _>("data").to_owned();
            let created_at = row.read::<i64, _>("created_at").to_owned();
            let updated_at = row.read::<i64, _>("updated_at").to_owned();

            Some(LeaderboardDto {
                id,
                year,
                board_id,
                data,
                created_at: DateTime::from_timestamp_secs(created_at).unwrap(),
                updated_at: DateTime::from_timestamp_secs(updated_at).unwrap(),
            })
        } else {
            None
        }
    }
}
