mod schema;

use diesel::{sql_query, SqliteConnection};
use diesel_async::RunQueryDsl;
use diesel_async::sync_connection_wrapper::SyncConnectionWrapper;
use diesel_async::pooled_connection::{AsyncDieselConnectionManager, PoolError};
use diesel_async::pooled_connection::bb8::{Pool, RunError};

type AsyncSqliteConnection = SyncConnectionWrapper<SqliteConnection>;

#[derive(Clone)]
pub struct ConnectionPool {
    pool: Pool<AsyncSqliteConnection>,
}

impl ConnectionPool {
    pub async fn open<S: Into<String>>(path: S) -> Result<Self, RunError> {
        let manager = AsyncDieselConnectionManager::<AsyncSqliteConnection>::new(path);
        let pool = Pool::builder()
            .build(manager)
            .await?;

        Ok(Self { pool })
    }

    pub async fn create_tables(&self) -> Result<(), RunError> {
        let mut conn = self.pool
            .get()
            .await?;

        let _ = sql_query(r#"
            CREATE TABLE IF NOT EXISTS players (
                internal_id INTEGER PRIMARY KEY,
                slap_id VARCHAR(40) NOT NULL UNIQUE
            );
        "#)
            .execute(&mut conn)
            .await
            .map_err(|err| RunError::User(PoolError::QueryError(err)))?;

        // TODO: add a timestamp
        let _ = sql_query(r#"
            CREATE TABLE IF NOT EXISTS matches (
                internal_id INTEGER PRIMARY KEY,
                match_id VAR_CHAR(40) NOT NULL UNIQUE
            );
        "#)
            .execute(&mut conn)
            .await
            .map_err(|err| RunError::User(PoolError::QueryError(err)))?;

        let _ = sql_query(r#"
            CREATE TABLE IF NOT EXISTS names (
                player_id INTEGER NOT NULL,
                name VARCHAR(32) NOT NULL,
                PRIMARY KEY (player_id, name),
                FOREIGN KEY (player_id) REFERENCES players(internal_id)
            );
        "#)
            .execute(&mut conn)
            .await
            .map_err(|err| RunError::User(PoolError::QueryError(err)))?;

        let _ = sql_query(r#"
            CREATE TABLE IF NOT EXISTS match_players (
                match_id INTEGER NOT NULL,
                player_id INTEGER NOT NULL,
                PRIMARY KEY (match_id, player_id),
                FOREIGN KEY (match_id) REFERENCES matches(internal_id),
                FOREIGN KEY (player_id) REFERENCES players(internal_id)
            );
        "#)
            .execute(&mut conn)
            .await
            .map_err(|err| RunError::User(PoolError::QueryError(err)))?;

        // TODO: create ranks table

        Ok(())
    }
}
