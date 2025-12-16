use chrono::DateTime;
use rusqlite::{params, Connection, Row};

use crate::model::game::{GameDto, GameMembershipDto};
use crate::model::leaderboard::{AocLeaderboardId, AocMemberId};

pub struct GameRepository;

impl TryFrom<&Row<'_>> for GameDto {
    type Error = rusqlite::Error;

    fn try_from(row: &Row) -> Result<Self, Self::Error> {
        let id: String = row.get("id")?;
        let leaderboard_id: i64 = row.get("leaderboard_id")?;
        let session_token: String = row.get("session_token")?;
        let created_at: i64 = row.get("created_at")?;
        let updated_at: i64 = row.get("updated_at")?;

        Ok(GameDto {
            id,
            leaderboard_id: leaderboard_id as AocLeaderboardId,
            session_token,
            created_at: DateTime::from_timestamp(created_at, 0).unwrap(),
            updated_at: DateTime::from_timestamp(updated_at, 0).unwrap(),
        })
    }
}

impl TryFrom<&Row<'_>> for GameMembershipDto {
    type Error = rusqlite::Error;

    fn try_from(row: &Row) -> Result<Self, Self::Error> {
        let id: i64 = row.get("id")?;
        let game_id: String = row.get("game_id")?;
        let member_id: i64 = row.get("member_id")?;
        let member_name: String = row.get("member_name")?;
        let created_at: i64 = row.get("created_at")?;

        Ok(GameMembershipDto {
            id: id as u32,
            game_id,
            member_id: member_id as AocMemberId,
            member_name,
            created_at: DateTime::from_timestamp(created_at, 0).unwrap(),
        })
    }
}

impl GameRepository {
    pub fn new() -> Self {
        GameRepository
    }

    /// Create a new game with the given ID
    pub fn create_game(
        &self,
        conn: &Connection,
        id: &str,
        leaderboard_id: u32,
        session_token: &str,
    ) -> Result<GameDto, rusqlite::Error> {
        let mut statement = conn.prepare(
            "INSERT INTO games (id, leaderboard_id, session_token)
             VALUES (?1, ?2, ?3)
             RETURNING *;",
        )?;
        let mut rows = statement.query(params![id, leaderboard_id as i64, session_token])?;

        if let Some(row) = rows.next()? {
            GameDto::try_from(row)
        } else {
            panic!("Expected game to be returned after insert.")
        }
    }

    /// Get a game by its ID
    pub fn get_game(&self, conn: &Connection, id: &str) -> Option<GameDto> {
        let mut statement = conn.prepare("SELECT * FROM games WHERE id = ?1;").ok()?;
        let mut rows = statement.query(params![id]).ok()?;
        rows.next().ok()?.and_then(|row| GameDto::try_from(row).ok())
    }

    /// Get all games (optional - for listing/debugging)
    pub fn get_all_games(&self, conn: &Connection) -> Vec<GameDto> {
        let mut statement = conn
            .prepare("SELECT * FROM games ORDER BY created_at DESC;")
            .unwrap();
        let rows = statement
            .query_map([], |row| GameDto::try_from(row))
            .unwrap();

        let mut games = Vec::new();
        for row_result in rows {
            match row_result {
                Ok(dto) => games.push(dto),
                Err(e) => {
                    eprintln!("Failed to convert row to GameDto: {:?}", e);
                }
            }
        }
        games
    }

    /// Create a game membership
    pub fn create_membership(
        &self,
        conn: &Connection,
        game_id: &str,
        member_id: u32,
        member_name: &str,
    ) -> Result<GameMembershipDto, rusqlite::Error> {
        let mut statement = conn.prepare(
            "INSERT INTO game_memberships (game_id, member_id, member_name)
             VALUES (?1, ?2, ?3)
             RETURNING *;",
        )?;
        let mut rows = statement.query(params![game_id, member_id as i64, member_name])?;

        if let Some(row) = rows.next()? {
            GameMembershipDto::try_from(row)
        } else {
            panic!("Expected membership to be returned after insert.")
        }
    }

    /// Delete a specific membership by its ID
    pub fn delete_membership(
        &self,
        conn: &Connection,
        membership_id: u32,
    ) -> Result<(), rusqlite::Error> {
        conn.execute("DELETE FROM game_memberships WHERE id = ?1;", params![membership_id as i64])?;
        Ok(())
    }

    /// Delete a membership by game_id and member_id
    pub fn delete_membership_by_game_and_member(
        &self,
        conn: &Connection,
        game_id: &str,
        member_id: u32,
    ) -> Result<(), rusqlite::Error> {
        conn.execute(
            "DELETE FROM game_memberships WHERE game_id = ?1 AND member_id = ?2;",
            params![game_id, member_id as i64],
        )?;
        Ok(())
    }

    /// Get all memberships for a game
    pub fn get_memberships_by_game(
        &self,
        conn: &Connection,
        game_id: &str,
    ) -> Vec<GameMembershipDto> {
        let mut statement = conn
            .prepare("SELECT * FROM game_memberships WHERE game_id = ?1 ORDER BY created_at ASC;")
            .unwrap();
        let rows = statement
            .query_map(params![game_id], |row| GameMembershipDto::try_from(row))
            .unwrap();

        let mut memberships = Vec::new();
        for row_result in rows {
            match row_result {
                Ok(dto) => memberships.push(dto),
                Err(e) => {
                    eprintln!("Failed to convert row to GameMembershipDto: {:?}", e);
                }
            }
        }
        memberships
    }
}
