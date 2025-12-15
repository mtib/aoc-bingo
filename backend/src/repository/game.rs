use chrono::DateTime;
use sqlite::{Connection, Row, Value};

use crate::model::game::{GameDto, GameMembershipDto};
use crate::model::leaderboard::{AocLeaderboardId, AocMemberId};

pub struct GameRepository;

impl TryFrom<Row> for GameDto {
    type Error = sqlite::Error;

    fn try_from(row: Row) -> Result<Self, Self::Error> {
        let id = row.try_read::<&str, _>("id")?.to_owned();
        let leaderboard_id: i64 = row.try_read("leaderboard_id")?;
        let session_token = row.try_read::<&str, _>("session_token")?.to_owned();
        let created_at: i64 = row.try_read("created_at")?;
        let updated_at: i64 = row.try_read("updated_at")?;

        Ok(GameDto {
            id,
            leaderboard_id: leaderboard_id as AocLeaderboardId,
            session_token,
            created_at: DateTime::from_timestamp(created_at, 0).unwrap(),
            updated_at: DateTime::from_timestamp(updated_at, 0).unwrap(),
        })
    }
}

impl TryFrom<Row> for GameMembershipDto {
    type Error = sqlite::Error;

    fn try_from(row: Row) -> Result<Self, Self::Error> {
        let id: i64 = row.try_read("id")?;
        let game_id = row.try_read::<&str, _>("game_id")?.to_owned();
        let member_id: i64 = row.try_read("member_id")?;
        let member_name = row.try_read::<&str, _>("member_name")?.to_owned();
        let created_at: i64 = row.try_read("created_at")?;

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
    ) -> Result<GameDto, sqlite::Error> {
        let mut statement = conn.prepare(
            "INSERT INTO games (id, leaderboard_id, session_token)
             VALUES (:id, :leaderboard_id, :session_token)
             RETURNING *;",
        )?;
        statement.bind::<&[(_, Value)]>(&[
            (":id", id.into()),
            (":leaderboard_id", (leaderboard_id as i64).into()),
            (":session_token", session_token.into()),
        ])?;

        match self.consume_statement_as_game(&mut statement) {
            Ok(Some(dto)) => Ok(dto),
            Ok(None) => panic!("Expected game to be returned after insert."),
            Err(e) => Err(e),
        }
    }

    /// Get a game by its ID
    pub fn get_game(&self, conn: &Connection, id: &str) -> Option<GameDto> {
        let mut statement = conn
            .prepare("SELECT * FROM games WHERE id = :id;")
            .unwrap();
        statement
            .bind::<&[(_, Value)]>(&[(":id", id.into())])
            .unwrap();
        self.consume_statement_as_game(&mut statement)
            .ok()
            .flatten()
    }

    /// Get all games (optional - for listing/debugging)
    pub fn get_all_games(&self, conn: &Connection) -> Vec<GameDto> {
        let mut statement = conn
            .prepare("SELECT * FROM games ORDER BY created_at DESC;")
            .unwrap();

        let mut games = Vec::new();
        for row_result in statement.iter() {
            match row_result {
                Ok(row) => match row.try_into() {
                    Ok(dto) => games.push(dto),
                    Err(e) => {
                        eprintln!("Failed to convert row to GameDto: {:?}", e);
                    }
                },
                Err(e) => {
                    eprintln!("Database error fetching game: {:?}", e);
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
    ) -> Result<GameMembershipDto, sqlite::Error> {
        let mut statement = conn.prepare(
            "INSERT INTO game_memberships (game_id, member_id, member_name)
             VALUES (:game_id, :member_id, :member_name)
             RETURNING *;",
        )?;
        statement.bind::<&[(_, Value)]>(&[
            (":game_id", game_id.into()),
            (":member_id", (member_id as i64).into()),
            (":member_name", member_name.into()),
        ])?;

        match self.consume_statement_as_membership(&mut statement) {
            Ok(Some(dto)) => Ok(dto),
            Ok(None) => panic!("Expected membership to be returned after insert."),
            Err(e) => Err(e),
        }
    }

    /// Delete a specific membership by its ID
    pub fn delete_membership(
        &self,
        conn: &Connection,
        membership_id: u32,
    ) -> Result<(), sqlite::Error> {
        let mut statement = conn.prepare("DELETE FROM game_memberships WHERE id = :id;")?;
        statement.bind::<&[(_, Value)]>(&[(":id", (membership_id as i64).into())])?;
        self.consume_statement(&mut statement)?;
        Ok(())
    }

    /// Get all memberships for a game
    pub fn get_memberships_by_game(
        &self,
        conn: &Connection,
        game_id: &str,
    ) -> Vec<GameMembershipDto> {
        let mut statement = conn
            .prepare("SELECT * FROM game_memberships WHERE game_id = :game_id ORDER BY created_at ASC;")
            .unwrap();
        statement
            .bind::<&[(_, Value)]>(&[(":game_id", game_id.into())])
            .unwrap();

        let mut memberships = Vec::new();
        for row_result in statement.iter() {
            match row_result {
                Ok(row) => match row.try_into() {
                    Ok(dto) => memberships.push(dto),
                    Err(e) => {
                        eprintln!("Failed to convert row to GameMembershipDto: {:?}", e);
                    }
                },
                Err(e) => {
                    eprintln!("Database error fetching membership: {:?}", e);
                }
            }
        }
        memberships
    }

    // Private helper methods
    fn consume_statement(&self, statement: &mut sqlite::Statement) -> Result<(), sqlite::Error> {
        while let Ok(state) = statement.next() {
            if state == sqlite::State::Done {
                break;
            }
        }
        Ok(())
    }

    fn consume_statement_as_game(
        &self,
        statement: &mut sqlite::Statement,
    ) -> Result<Option<GameDto>, sqlite::Error> {
        let result = match statement.iter().next() {
            None => Ok(None),
            Some(row) => match row {
                Ok(row) => match row.try_into() {
                    Ok(dto) => Ok(Some(dto)),
                    Err(e) => {
                        eprintln!("Failed to convert row to GameDto: {:?}", e);
                        Err(e)
                    }
                },
                Err(e) => {
                    eprintln!("Database error fetching game: {:?}", e);
                    Err(e)
                }
            },
        };
        self.consume_statement(statement)?;
        result
    }

    fn consume_statement_as_membership(
        &self,
        statement: &mut sqlite::Statement,
    ) -> Result<Option<GameMembershipDto>, sqlite::Error> {
        let result = match statement.iter().next() {
            None => Ok(None),
            Some(row) => match row {
                Ok(row) => match row.try_into() {
                    Ok(dto) => Ok(Some(dto)),
                    Err(e) => {
                        eprintln!("Failed to convert row to GameMembershipDto: {:?}", e);
                        Err(e)
                    }
                },
                Err(e) => {
                    eprintln!("Database error fetching membership: {:?}", e);
                    Err(e)
                }
            },
        };
        self.consume_statement(statement)?;
        result
    }
}
