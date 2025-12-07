use chrono::{DateTime, Utc};
use sqlite::{Connection, Row, Value};

pub struct LeaderboardRepository;

pub struct LeaderboardDto {
    pub id: i64,
    pub year: i64,
    pub board_id: i64,
    pub data: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

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
        statement
            .into_iter()
            .next()
            .and_then(|result| match result {
                Ok(row) => match row.try_into() {
                    Ok(dto) => Some(dto),
                    Err(e) => {
                        eprintln!(
                            "Failed to convert row to LeaderboardDto for year={}, board_id={}: {:?}",
                            year, board_id, e
                        );
                        None
                    }
                },
                Err(e) => {
                    eprintln!(
                        "Database error fetching leaderboard for year={}, board_id={}: {:?}",
                        year, board_id, e
                    );
                    None
                }
            })
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
}
