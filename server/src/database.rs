use std::path::Path;
use async_sqlite::{Error, Pool, PoolBuilder};
use async_sqlite::rusqlite::params;

#[derive(Clone)]
pub struct ConnectionPool {
    pool: Pool,
}

impl ConnectionPool {
    pub async fn open<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let pool = PoolBuilder::new()
            .path(path)
            .open().await?;

        Ok(Self { pool })
    }

    pub async fn create_tables(&self) -> Result<(), Error> {
        self.pool.conn(|conn| conn.execute(r#"
            CREATE TABLE IF NOT EXISTS players (
                internal_id INTEGER PRIMARY KEY,
                slap_id VARCHAR(40) NOT NULL UNIQUE
            );
        "#, params![])).await?;

        // TODO: add a timestamp
        self.pool.conn(|conn| conn.execute(r#"
            CREATE TABLE IF NOT EXISTS matches (
                internal_id INTEGER PRIMARY KEY,
                match_id VAR_CHAR(40) NOT NULL UNIQUE
            );
        "#, params![])).await?;

        self.pool.conn(|conn| conn.execute(r#"
            CREATE TABLE IF NOT EXISTS names (
                player_id INT NOT NULL,
                name VARCHAR(32) NOT NULL,
                PRIMARY KEY (player_id, name),
                FOREIGN KEY (player_id) REFERENCES players(internal_id)
            );
        "#, params![])).await?;

        self.pool.conn(|conn| conn.execute(r#"
            CREATE TABLE IF NOT EXISTS match_players (
                match_id INT NOT NULL,
                player_id INT NOT NULL,
                PRIMARY KEY (match_id, player_id),
                FOREIGN KEY (match_id) REFERENCES matches(internal_id),
                FOREIGN KEY (player_id) REFERENCES players(internal_id)
            );
        "#, params![])).await?;

        // TODO: create ranks table

        Ok(())
    }
}
