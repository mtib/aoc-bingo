use std::fs;

use chrono::DateTime;
use include_dir::Dir;
use rusqlite::params;

use super::pool::{DbConnection, DbPool, create_pool};

pub struct DatabaseManager {
    pool: DbPool,
}

static MIGRATION_DIR: Dir<'static> =
    include_dir::include_dir!("$CARGO_MANIFEST_DIR/src/db/migrations");

#[derive(thiserror::Error, Debug)]
pub enum DbError {
    #[error("Unexpected error: {0}")]
    #[allow(dead_code)]
    Unexpected(String),
    #[error("SQLite error: {0}")]
    SqliteError(#[from] rusqlite::Error),
    #[error("Fs error: {0}")]
    FsError(#[from] std::io::Error),
    #[error("Pool error: {0}")]
    PoolError(#[from] r2d2::Error),
}

impl DatabaseManager {
    pub fn new(db_path: &str) -> Result<Self, DbError> {
        fs::create_dir_all("./data").map_err(DbError::FsError)?;
        let pool = create_pool(db_path)?;
        Ok(DatabaseManager { pool })
    }

    pub fn get_connection(&self) -> Result<DbConnection, DbError> {
        self.pool.get().map_err(DbError::PoolError)
    }

    pub fn get_pool(&self) -> &DbPool {
        &self.pool
    }

    fn setup_migration_table(&self) -> Result<(), DbError> {
        println!("Setting up migration table...");
        let conn = self.get_connection()?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS migrations (
                id TEXT PRIMARY KEY,
                applied_at INTEGER DEFAULT (unixepoch())
            );",
            [],
        )?;
        Ok(())
    }

    fn get_applied_migrations(&self) -> Result<Vec<String>, DbError> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare("SELECT id FROM migrations ORDER BY id;")?;
        let rows = stmt.query_map([], |row| row.get(0))?;
        let mut applied_migrations = Vec::new();
        for id_result in rows {
            applied_migrations.push(id_result?);
        }
        Ok(applied_migrations)
    }

    fn apply_migration(&self, migration_id: &str, migration_sql: &str) {
        println!("Applying migration: {}", migration_id);
        let conn = self.get_connection().unwrap();
        conn.execute_batch(migration_sql).unwrap();

        let mut statement = conn
            .prepare("INSERT INTO migrations (id) VALUES (?1) RETURNING *;")
            .unwrap();
        let mut rows = statement.query(params![migration_id]).unwrap();

        if let Some(row) = rows.next().unwrap() {
            let applied_id: String = row.get(0).unwrap();
            let applied_at_secs: i64 = row.get(1).unwrap();
            let applied_at = DateTime::from_timestamp(applied_at_secs, 0).unwrap();
            println!(
                "Migration applied: id={}, applied_at={}",
                applied_id, applied_at
            );
        }
    }

    pub fn init(&self) {
        self.setup_migration_table().unwrap();

        let migrations = {
            let mut migrations = MIGRATION_DIR.files().collect::<Box<_>>();
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
