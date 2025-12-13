use rocket::{http::Status, post, serde::json::Json};

use crate::{
    model::leaderboard::{
        AocMemberId, LeaderboardDto, ShuffleLeaderboardDataDto, ShuffleLeaderboardDayDto,
        ShuffleLeaderboardDto,
    },
    service::LeaderboardService,
};

#[derive(serde::Deserialize)]
pub struct LeaderboardRequest {
    year: u32,
    board_id: u32,
    session_token: String,
}

#[post("/", data = "<req>")]
pub async fn index(
    req: Json<LeaderboardRequest>,
) -> Result<Json<LeaderboardDto>, (Status, String)> {
    let req = req.into_inner();

    let result = {
        let lbs = LeaderboardService::new();
        lbs.get_or_create_leaderboard(req.year, req.board_id, Some(&req.session_token))
            .await
    };

    match result {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err((Status::InternalServerError, e.to_string())),
    }
}

#[derive(serde::Deserialize)]
pub struct BingoAllRequest {
    board_id: u32,
    session_token: String,
    member_ids: Vec<AocMemberId>,
    /// Optional difficulty filter: 0.1 (easy) to 0.9 (hard)
    ///
    /// if < 0.5 => uses `1 - difficulty` as chance to skip hard puzzles
    /// if >= 0.5 => uses `difficulty` as chance to skip easy puzzles
    difficulty: Option<f32>,
}

#[post("/bingo/all", data = "<req>")]
pub async fn bingo_all(
    req: Json<BingoAllRequest>,
) -> Result<Json<ShuffleLeaderboardDto>, (Status, String)> {
    let req = req.into_inner();

    let puzzles_result = {
        let lbs = LeaderboardService::new();
        lbs.get_bingo_options(
            None,
            req.board_id,
            Some(&req.session_token),
            Some(&req.member_ids),
        )
        .await
    };

    let puzzles = match puzzles_result {
        Ok(leaderboard) => leaderboard,
        Err(e) => return Err((Status::BadRequest, e.to_string())),
    };

    let result = {
        let mut data = ShuffleLeaderboardDataDto {
            players: vec![],
            days: vec![],
            members: std::collections::HashMap::new(),
        };
        for puzzle in puzzles {
            data.days.push(ShuffleLeaderboardDayDto {
                year: puzzle.date.year,
                day: puzzle.date.day,
                part: puzzle.part.into(),
            });
        }
        ShuffleLeaderboardDto {
            board_id: req.board_id,
            data,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    };

    Ok(Json(result))
}
