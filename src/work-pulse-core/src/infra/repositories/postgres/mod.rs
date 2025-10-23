pub mod accounting_categories_list;
pub mod activities_list;

use sqlx::PgPool;

/// A wrapper around a PostgreSQL connection pool.
#[derive(Clone)]
pub struct PsqlConnection {
    pool: PgPool,
}

impl PsqlConnection {
    /// Creates a new `PsqlConnection` instance.
    ///
    /// # Arguments
    ///
    /// - `pool`: A PostgreSQL connection pool.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Creates a new `PsqlConnection` instance with the given database URL.
    ///
    /// # Arguments
    ///
    /// - `database_url`: The database URL to connect to.
    pub async fn with_database_url(database_url: &str) -> Self {
        let pool = PgPool::connect(database_url).await.unwrap();
        Self::new(pool)
    }

    /// Returns a reference to the underlying PostgreSQL connection pool.
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}
