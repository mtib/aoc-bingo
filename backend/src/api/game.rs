use rocket::{http::Status, post, serde::json::Json};
use serde::Deserialize;

use crate::{model::game::GameDto, service::GameService};

#[derive(Deserialize)]
pub struct CreateGameRequest {
    pub leaderboard_id: u32,
    pub session_token: String,
}

#[derive(serde::Serialize)]
pub struct CreateGameResponse {
    pub game: GameDto,
}

/// POST /game - Create a new game and return the generated game ID
#[post("/", data = "<req>")]
pub async fn create(
    req: Json<CreateGameRequest>,
) -> Result<Json<CreateGameResponse>, (Status, String)> {
    let req = req.into_inner();

    let service = GameService::new();
    match service.create_game(req.leaderboard_id, &req.session_token, 10) {
        Ok(game) => Ok(Json(CreateGameResponse { game })),
        Err(e) => Err((Status::InternalServerError, e.to_string())),
    }
}
