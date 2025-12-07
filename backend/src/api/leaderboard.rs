use rocket::{http::Status, post, serde::json::Json};

use crate::{model::leaderboard::LeaderboardDto, service::LeaderboardService};

#[derive(serde::Deserialize)]
pub struct LeaderboardRequest {
    year: u32,
    board_id: u32,
    session_token: String,
}

#[post("/", data = "<req>")]
pub async fn index(req: Json<LeaderboardRequest>) -> Result<Json<LeaderboardDto>, Status> {
    let req = req.into_inner();

    let result = {
        let lbs = LeaderboardService::new();
        lbs.get_or_create_leaderboard(req.year, req.board_id, Some(&req.session_token))
            .await
    };

    match result {
        Ok(response) => Ok(Json(response)),
        Err(_) => Err(rocket::http::Status::InternalServerError),
    }
}
