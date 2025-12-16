use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Utc};
use rocket::{State, delete, get, http::Status, post, serde::json::Json};
use serde::Deserialize;

use crate::{
    db::DbPool,
    model::{
        aoc::{AocPart, AocPuzzle},
        game::{GameDto, GameLeaderboardMemberDto, GameMembershipDto},
        leaderboard::{AocMemberId, Day, Year},
    },
    service::{
        GameService, LeaderboardService,
        game::{GameError, GameMembershipError},
    },
};

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
    pool: &State<DbPool>,
    req: Json<CreateGameRequest>,
) -> Result<Json<CreateGameResponse>, (Status, String)> {
    let req = req.into_inner();
    let conn = pool
        .get()
        .map_err(|e| (Status::InternalServerError, e.to_string()))?;

    let service = GameService::new();
    match service.create_game(&conn, req.leaderboard_id, &req.session_token, 10) {
        Ok(game) => Ok(Json(CreateGameResponse { game })),
        Err(e) => Err((Status::InternalServerError, e.to_string())),
    }
}

#[derive(serde::Serialize)]
pub struct GetGameMembersResponse {
    pub possible_members: Vec<GameLeaderboardMemberDto>,
    pub members: Vec<GameMembershipDto>,
}

fn map_game_error(e: GameError) -> (Status, String) {
    match e {
        GameError::DatabaseError(_) | GameError::IdGenerationFailed(_) => {
            (Status::InternalServerError, e.to_string())
        }
        GameError::NotFound(_) | GameError::LeaderboardNotFound => {
            (Status::NotFound, e.to_string())
        }
    }
}

#[get("/<id>/members")]
pub async fn get_members(
    pool: &State<DbPool>,
    id: &str,
) -> Result<Json<GetGameMembersResponse>, (Status, String)> {
    let service = GameService::new();
    let possible_members = service
        .get_possible_members(pool, id)
        .await
        .map_err(map_game_error)?;

    let mut conn = pool
        .get()
        .map_err(|e| (Status::InternalServerError, e.to_string()))?;
    let members = service
        .get_memberships(&mut conn, id)
        .map_err(map_game_error)?;

    Ok(Json(GetGameMembersResponse {
        possible_members: possible_members,
        members,
    }))
}

#[derive(serde::Serialize)]
pub struct GetAllPuzzlesResponse {
    pub puzzles: Vec<AocPuzzle>,
    pub members: Vec<GameMembershipDto>,
    pub game_id: String,
}

#[get("/<id>/puzzles/all")]
pub async fn get_all_puzzles(
    pool: &State<DbPool>,
    id: &str,
) -> Result<Json<GetAllPuzzlesResponse>, (Status, String)> {
    let service = GameService::new();

    // Get game and members (sync work)
    let (game, members) = {
        let mut conn = pool
            .get()
            .map_err(|e| (Status::InternalServerError, e.to_string()))?;
        let game = service.get_game(&conn, id).map_err(map_game_error)?;
        let members = service
            .get_memberships(&mut conn, id)
            .map_err(map_game_error)?;
        (game, members)
    };

    // Get bingo options (async work)
    let lbs = LeaderboardService::new();
    let options = lbs
        .get_bingo_options(
            pool,
            None,
            game.leaderboard_id,
            Some(&game.session_token),
            Some(
                members
                    .iter()
                    .map(|m| m.member_id)
                    .collect::<Vec<_>>()
                    .as_slice(),
            ),
            Some(game.created_at),
        )
        .await
        .map_err(|e| (Status::InternalServerError, e.to_string()))?;

    Ok(Json(GetAllPuzzlesResponse {
        puzzles: options,
        members,
        game_id: game.id,
    }))
}

/// For all members in a game, return all the puzzles in get_all_puzzles for that game that they now completed
#[get("/<id>/completion")]
pub async fn get_completion(
    pool: &State<DbPool>,
    id: &str,
) -> Result<
    Json<HashMap<AocMemberId, HashSet<(Year, Day, AocPart, DateTime<Utc>)>>>,
    (Status, String),
> {
    let service = GameService::new();

    // Get game and members (sync work)
    let (game, members) = {
        let mut conn = pool
            .get()
            .map_err(|e| (Status::InternalServerError, e.to_string()))?;
        let game = service.get_game(&conn, id).map_err(map_game_error)?;
        let members = service
            .get_memberships(&mut conn, id)
            .map_err(map_game_error)?;
        (game, members)
    };

    // Get bingo options (async work)
    let lbs = LeaderboardService::new();
    let options = lbs
        .get_bingo_options(
            pool,
            None,
            game.leaderboard_id,
            Some(&game.session_token),
            Some(
                members
                    .iter()
                    .map(|m| m.member_id)
                    .collect::<Vec<_>>()
                    .as_slice(),
            ),
            Some(game.created_at),
        )
        .await
        .map_err(|e| (Status::InternalServerError, e.to_string()))?;
    let current_leaderboards = lbs
        .get_or_create_all_leaderboards(pool, game.leaderboard_id, Some(&game.session_token))
        .await;

    let mut completions: HashMap<AocMemberId, HashSet<(Year, Day, AocPart, DateTime<Utc>)>> =
        HashMap::new();

    for leaderboard in current_leaderboards.into_iter().filter_map(|r| r.ok()) {
        for member in leaderboard.data.members.values() {
            if !members.iter().any(|m| m.member_id == member.id) {
                continue;
            }
            for (&day, day_completion) in member.completion_day_level.iter() {
                for (&part, star_info) in day_completion.iter() {
                    if !options.iter().any(|option| {
                        AocPart::from(part) == option.part
                            && option.date.year == leaderboard.year
                            && option.date.day == day
                    }) {
                        continue;
                    }
                    completions
                        .entry(member.id)
                        .or_insert_with(HashSet::new)
                        .insert((
                            leaderboard.year as Year,
                            day,
                            AocPart::from(part),
                            DateTime::<Utc>::from_timestamp(star_info.get_star_ts as i64, 0)
                                .unwrap(),
                        ));
                }
            }
        }
    }

    Ok(Json(completions))
}

#[derive(Deserialize)]
pub struct CreateMembershipRequest {
    pub member_id: u32,
    pub member_name: String,
}

#[derive(serde::Serialize)]
pub struct CreateMembershipResponse {
    pub membership: GameMembershipDto,
}

/// POST /game/<id>/members - Add a member to a game
#[post("/<id>/members", data = "<req>")]
pub async fn create_membership(
    pool: &State<DbPool>,
    id: &str,
    req: Json<CreateMembershipRequest>,
) -> Result<Json<CreateMembershipResponse>, (Status, String)> {
    let req = req.into_inner();
    let mut conn = pool
        .get()
        .map_err(|e| (Status::InternalServerError, e.to_string()))?;
    let service = GameService::new();

    match service.create_membership(&mut conn, id, req.member_id, &req.member_name) {
        Ok(membership) => Ok(Json(CreateMembershipResponse { membership })),
        Err(GameMembershipError::GameNotFound(_)) => {
            Err((Status::NotFound, "Game not found".to_string()))
        }
        Err(e) => Err((Status::InternalServerError, e.to_string())),
    }
}

/// DELETE /game/<game_id>/members/<member_id> - Remove a member from a game
#[delete("/<game_id>/members/<member_id>")]
pub async fn delete_membership(
    pool: &State<DbPool>,
    game_id: &str,
    member_id: u32,
) -> Result<Status, (Status, String)> {
    let mut conn = pool
        .get()
        .map_err(|e| (Status::InternalServerError, e.to_string()))?;
    let service = GameService::new();

    match service.delete_membership_by_game_and_member(&mut conn, game_id, member_id) {
        Ok(_) => Ok(Status::NoContent),
        Err(GameMembershipError::NotFound(_)) | Err(GameMembershipError::GameNotFound(_)) => {
            Err((Status::NotFound, "Game or membership not found".to_string()))
        }
        Err(e) => Err((Status::InternalServerError, e.to_string())),
    }
}
