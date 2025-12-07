use chrono::{DateTime, Utc};

use crate::client::model::leaderboard::LeaderboardResponse;

#[derive(Debug, Clone, serde::Serialize)]
pub struct LeaderboardDto {
    pub id: i64,
    pub year: i64,
    pub board_id: i64,
    pub data: LeaderboardResponse,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
