use r2d2_sqlite::SqliteConnectionManager;

pub type DbPool = r2d2::Pool<SqliteConnectionManager>;
pub type DbConnection = r2d2::PooledConnection<SqliteConnectionManager>;

pub fn create_pool(db_path: &str) -> Result<DbPool, r2d2::Error> {
    let manager = SqliteConnectionManager::file(db_path).with_init(|conn| {
        // Enable WAL mode for better concurrency (readers don't block writers)
        conn.execute_batch(
            "PRAGMA journal_mode=WAL;
             PRAGMA busy_timeout=5000;
             PRAGMA cache_size=-64000;
             PRAGMA synchronous=NORMAL;
             PRAGMA foreign_keys=ON;
             PRAGMA temp_store=MEMORY;",
        )?;
        Ok(())
    });

    r2d2::Pool::builder()
        .max_size(16) // Reasonable for SQLite with WAL mode
        .connection_timeout(std::time::Duration::from_secs(30))
        .build(manager)
}
