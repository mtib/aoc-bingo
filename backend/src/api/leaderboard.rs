use rocket::{http::Status, post, serde::json::Json};

use crate::{
    client::{AocClient, model::leaderboard::LeaderboardResponse},
    repository::LeaderboardRepository,
};

#[derive(serde::Deserialize)]
pub struct LeaderboardRequest {
    year: u32,
    board_id: u32,
    session_token: String,
}

#[post("/", data = "<req>")]
pub async fn index(req: Json<LeaderboardRequest>) -> Result<Json<LeaderboardResponse>, Status> {
    let req = req.into_inner();
    let dbm = crate::db::DatabaseManager::new();
    let lbr = LeaderboardRepository::new();

    let cached = lbr.get_leaderboard(dbm.get_connection(), req.year, req.board_id);

    if let Some(leaderboard) = cached {
        return Ok(Json(
            serde_json::from_str(&leaderboard.data).map_err(|_| Status::InternalServerError)?,
        ));
    }

    let client = AocClient::new();
    let result = client
        .fetch_leaderboard(req.year, req.board_id, &req.session_token)
        .await;

    if let Ok(ref response) = result {
        let r = lbr.save_leaderboard(
            dbm.get_connection(),
            req.year,
            req.board_id,
            serde_json::to_string(response)
                .map_err(|_| rocket::http::Status::InternalServerError)?
                .as_str(),
        );

        match r {
            Ok(_) => println!("Leaderboard cached successfully."),
            Err(err) => println!("Failed to cache leaderboard. {:?}", err),
        }
    }

    match result {
        Ok(response) => Ok(Json(response)),
        Err(_) => Err(rocket::http::Status::InternalServerError),
    }
}
