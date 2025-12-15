use chrono::{DateTime, Utc};
use crate::model::leaderboard::{AocLeaderboardId, AocMemberId};

/// 8-character alphanumeric game ID
pub type GameId = String;

#[derive(Debug, Clone, serde::Serialize)]
pub struct GameDto {
    pub id: GameId,
    pub leaderboard_id: AocLeaderboardId,
    pub session_token: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct GameMembershipDto {
    pub id: u32,
    pub game_id: GameId,
    pub member_id: AocMemberId,
    pub member_name: String,
    pub created_at: DateTime<Utc>,
}
