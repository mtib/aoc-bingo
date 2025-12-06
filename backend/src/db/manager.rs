use std::fs;

use include_dir::Dir;
use sqlite::Value;

pub struct DatabaseManager {
    connection: sqlite::Connection,
}

static MIGRATION_DIR: Dir<'static> =
    include_dir::include_dir!("$CARGO_MANIFEST_DIR/src/db/migrations");

#[derive(thiserror::Error, Debug)]
pub enum DbError {
    #[error("Unexpected error: {0}")]
    #[allow(dead_code)]
    Unexpected(String),
    #[error("SQLite error: {0}")]
    SqliteError(#[from] sqlite::Error),
    #[error("Fs error: {0}")]
    FsError(#[from] std::io::Error),
}

impl DatabaseManager {
    pub fn new() -> Self {
        DatabaseManager {
            connection: DatabaseManager::open_connection().unwrap(),
        }
    }

    fn open_connection() -> Result<sqlite::Connection, DbError> {
        fs::create_dir_all("./data").map_err(DbError::FsError)?;
        let db_path = "./data/db.sqlite";
        let db_exists = fs::metadata(db_path).is_ok();
        let conn = sqlite::open(db_path).map_err(DbError::SqliteError)?;
        if !db_exists {
            conn.execute("PRAGMA journal_mode=WAL;")?;
        }
        Ok(conn)
    }

    fn setup_migration_table(&self) -> Result<(), DbError> {
        println!("Setting up migration table...");
        self.connection
            .execute(
                "CREATE TABLE IF NOT EXISTS migrations (
                id TEXT PRIMARY KEY,
                applied_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );",
            )
            .map_err(DbError::SqliteError)?;
        Ok(())
    }

    fn get_applied_migrations(&self) -> Result<Vec<String>, DbError> {
        let mut stmt = self
            .connection
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
        println!("Applying migration: {}", migration_id);
        self.connection.execute(migration_sql).unwrap();

        let mut statement = self
            .connection
            .prepare("INSERT INTO migrations (id) VALUES (:id) RETURNING *;")
            .unwrap();
        statement
            .bind::<&[(_, Value)]>(&[(":id", migration_id.into())])
            .unwrap();
        for row in statement.iter() {
            if row.is_err() {
                continue;
            }
            let row = row.unwrap();
            let applied_id = row.read::<&str, _>("id");
            let applied_at = row.read::<&str, _>("applied_at");
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
