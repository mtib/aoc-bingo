use rusqlite::Transaction;

use super::pool::DbConnection;

/// Execute a function within a transaction
/// The transaction is automatically committed if the function returns Ok,
/// and automatically rolled back if the function returns Err or panics
pub fn with_transaction<F, R, E>(conn: &mut DbConnection, f: F) -> Result<R, E>
where
    F: FnOnce(&Transaction) -> Result<R, E>,
    E: From<rusqlite::Error>,
{
    let tx = conn.transaction()?;
    let result = f(&tx)?;
    tx.commit()?;
    Ok(result)
}
