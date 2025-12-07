use chrono::{DateTime, Utc};

#[derive(Debug, Clone, serde::Serialize)]
pub struct LeaderboardDto {
    pub id: i64,
    pub year: i64,
    pub board_id: i64,
    pub data: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
