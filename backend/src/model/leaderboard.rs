use std::collections::HashMap;

use chrono::{DateTime, Utc};

use crate::client::model::leaderboard::LeaderboardResponse;

/// The unique id for a leaderboard year entry on the official advent of code website.
pub type AocLeaderboardYearId = u32;
/// The unique id for a leaderboard on the official advent of code website. (Ranges multiple years)
pub type AocLeaderboardId = u32;

/// Id of a member (user of leaderboard) on the official advent of code website.
pub type AocMemberId = u32;

pub type Year = u32;
pub type Day = u32;
pub type Part = u32;

#[derive(Debug, Clone, serde::Serialize)]
pub struct LeaderboardDto {
    pub id: AocLeaderboardYearId,
    pub year: Year,
    pub board_id: AocLeaderboardId,
    pub data: LeaderboardResponse,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ShuffleLeaderboardDto {
    pub board_id: AocLeaderboardId,
    pub data: ShuffleLeaderboardDataDto,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ShuffleMemberDto {
    pub id: AocMemberId,
    pub name: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ShuffleLeaderboardDataDto {
    pub players: Vec<ShuffleMemberDto>,
    pub days: Vec<ShuffleLeaderboardDayDto>,
    pub members: HashMap<AocMemberId, ShuffleMemberDataDto>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ShuffleLeaderboardDayDto {
    pub year: Year,
    pub day: Day,
    pub part: Part,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ShuffleMemberDataDto {
    pub day: ShuffleLeaderboardDayDto,
    pub completed_at: DateTime<Utc>,
}
