use rocket::{http::Status, post, serde::json::Json};

use crate::client::{AocClient, model::leaderboard::LeaderboardResponse};

#[derive(serde::Deserialize)]
pub struct LeaderboardRequest {
    year: u32,
    board_id: u32,
    session_token: String,
}

#[post("/", data = "<req>")]
pub async fn index(req: Json<LeaderboardRequest>) -> Result<Json<LeaderboardResponse>, Status> {
    let req = req.into_inner();
    let client = AocClient::new();
    let result = client
        .fetch_leaderboard(req.year, req.board_id, &req.session_token)
        .await
        .map(Json);

    match result {
        Ok(json) => Ok(json),
        Err(_) => Err(rocket::http::Status::InternalServerError),
    }
}
