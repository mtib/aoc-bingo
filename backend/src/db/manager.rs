use std::{error::Error, fs};

use include_dir::Dir;
use sqlite::Value;

pub struct DatabaseManager {}

static MIGRATION_DIR: Dir<'static> =
    include_dir::include_dir!("$CARGO_MANIFEST_DIR/src/db/migrations");

#[derive(thiserror::Error, Debug)]
pub enum DbError {
    #[error("Unexpected error: {0}")]
    Unexpected(String),
    #[error("SQLite error: {0}")]
    SqliteError(#[from] sqlite::Error),
    #[error("Fs error: {0}")]
    FsError(#[from] std::io::Error),
}

impl DatabaseManager {
    pub fn new() -> Self {
        // Initialize the database connection here
        DatabaseManager {
            // ...
        }
    }

    pub fn get_connection(&self) -> Result<sqlite::Connection, DbError> {
        // Return a database connection
        fs::create_dir_all("./data").map_err(DbError::FsError)?;
        sqlite::open("./data/db.sqlite").map_err(DbError::SqliteError)
    }

    fn setup_migration_table(&self) -> Result<(), DbError> {
        println!("Setting up migration table...");
        let conn = self.get_connection()?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS migrations (
                id TEXT PRIMARY KEY,
                applied_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );",
        )
        .map_err(DbError::SqliteError)?;
        Ok(())
    }

    fn get_applied_migrations(&self) -> Result<Vec<String>, DbError> {
        let conn = self.get_connection()?;
        let mut stmt = conn
            .prepare("SELECT id FROM migrations ORDER BY id;")
            .map_err(DbError::SqliteError)?;
        let mut rows = stmt.iter();
        let mut applied_migrations = Vec::new();
        while let Some(Ok(row)) = rows.next() {
            let id = row.read::<&str, _>("id").to_owned();
            applied_migrations.push(id);
        }
        Ok(applied_migrations)
    }

    fn apply_migration(&self, migration_id: &str, migration_sql: &str) {
        // Apply the given migration SQL to the database
        println!("Applying migration: {}", migration_id);
        let conn = self.get_connection().unwrap();
        // TODO only apply new migrations
        conn.execute(migration_sql).unwrap();

        let mut statement = conn
            .prepare("INSERT INTO migrations (id) VALUES (:id);")
            .unwrap();
        statement
            .bind::<&[(_, Value)]>(&[(":id", migration_id.into())])
            .unwrap();
        statement.next().unwrap();
    }

    pub fn init(&self) {
        self.setup_migration_table().unwrap();

        let migrations = {
            let mut migrations = MIGRATION_DIR.files().collect::<Vec<_>>();
            migrations.sort_by_key(|f| f.path().file_name().unwrap().to_str().unwrap());
            migrations
        };

        let applied_migrations = self.get_applied_migrations().unwrap();

        for file in migrations {
            let migration_id = file.path().file_name().unwrap().to_str().unwrap();
            if applied_migrations.iter().any(|a| a == migration_id) {
                println!("Skipping already applied migration: {}", migration_id);
                continue;
            }
            let migration_sql = file.contents_utf8().unwrap();
            self.apply_migration(migration_id, migration_sql);
        }
    }
}
