use std::marker::Send;
use time::OffsetDateTime;
use diesel::prelude::*;
use diesel::dsl::*;
use diesel::sqlite::Sqlite;
use diesel::result::Error;
use diesel_async::{AsyncConnection, RunQueryDsl};
use diesel_async::sync_connection_wrapper::SyncConnectionWrapper;
use diesel_async::pooled_connection::bb8::PooledConnection;
use diesel_async::scoped_futures::ScopedFutureExt;
use crate::types::string::*;
use crate::types::integer::*;
use crate::types::slapshot::Player;
use crate::types::db::{NewPlayerRow, PlayerRow, NewNameRow, NameRow};

pub type AsyncSqliteConnection = SyncConnectionWrapper<SqliteConnection>;
pub type PooledSqliteConnection<'a> = PooledConnection<'a, AsyncSqliteConnection>;

pub trait DatabaseConnection {
    async fn add_player_id(&mut self, player_id: &PlayerId) -> Result<PlayerRow, Error>;

    async fn get_player(&mut self, id: &PlayerId) -> Result<PlayerRow, Error>;

    async fn add_player(&mut self, player: &Player) -> Result<NameRow, Error>;

    async fn add_or_update_player_name(
        &mut self,
        player_row: &PlayerRow,
        username: &Username,
    ) -> Result<NameRow, Error>;

    async fn get_player_names(
        &mut self,
        player_row: &PlayerRow,
    ) -> Result<Vec<NameRow>, Error>;

    async fn get_all_player_names(
        &mut self,
    ) -> Result<Vec<(PlayerRow, Vec<NameRow>)>, Error>;

    async fn create_tables(&mut self) -> Result<(), Error>;
}

impl<C> DatabaseConnection for C
where C: Send + AsyncConnection<Backend = Sqlite>,
{
    async fn add_player_id(&mut self, player_id: &PlayerId) -> Result<PlayerRow, Error> {
        use crate::db::schema::players::dsl::*;

        let new_player = NewPlayerRow {
            slap_id: PlayerId::clone(player_id),
        };

        insert_into(players)
            .values(&new_player)
            .returning(PlayerRow::as_returning())
            .get_result(self)
            .await
    }

    async fn get_player(&mut self, id: &PlayerId) -> Result<PlayerRow, Error> {
        use crate::db::schema::players::dsl::*;

        players
            .filter(slap_id.eq(id))
            .select(PlayerRow::as_select())
            .get_result(self)
            .await
    }

    async fn add_player(&mut self, player: &Player) -> Result<NameRow, Error> {
        self.transaction(|conn| async move {
            let player_row = conn.add_player_id(&player.game_user_id).await?;
            conn.add_or_update_player_name(&player_row, &player.username).await
        }.scope_boxed()).await
    }

    async fn add_or_update_player_name(
        &mut self,
        player_row: &PlayerRow,
        username: &Username,
    ) -> Result<NameRow, Error> {
        use crate::db::schema::names::dsl::*;

        let new_name = NewNameRow {
            player_id: player_row.internal_id,
            name: Username::clone(username),
        };

        // insert the new name or update the last used timestamp if it exists
        insert_into(names)
            .values(&new_name)
            .on_conflict((player_id, name))
            .do_update()
            .set(last_used.eq(OffsetDateTime::now_utc()))
            .returning(NameRow::as_returning())
            .get_result(self)
            .await
    }

    async fn get_player_names(
        &mut self,
        player_row: &PlayerRow,
    ) -> Result<Vec<NameRow>, Error> {
        NameRow::belonging_to(player_row)
            .select(NameRow::as_select())
            .load(self)
            .await
    }

    async fn get_all_player_names(
        &mut self,
    ) -> Result<Vec<(PlayerRow, Vec<NameRow>)>, Error> {
        use crate::db::schema::players;

        let all_players = players::table
            .select(PlayerRow::as_select())
            .load(self)
            .await?;

        let all_names = NameRow::belonging_to(&all_players)
            .select(NameRow::as_select())
            .load(self)
            .await?;

        let player_names = all_names
            .grouped_by(&all_players)
            .into_iter()
            .zip(all_players)
            .map(|(names, player)| (player, names))
            .collect::<Vec<(PlayerRow, Vec<NameRow>)>>();

        Ok(player_names)
    }

    async fn create_tables(&mut self) -> Result<(), Error> {
        let _ = sql_query(r#"
            CREATE TABLE IF NOT EXISTS players (
                internal_id INTEGER PRIMARY KEY AUTOINCREMENT,
                slap_id VARCHAR(40) NOT NULL UNIQUE
            );
        "#)
            .execute(self)
            .await?;

        // TODO: add a timestamp
        let _ = sql_query(r#"
            CREATE TABLE IF NOT EXISTS matches (
                internal_id INTEGER PRIMARY KEY AUTOINCREMENT,
                match_id VAR_CHAR(40) NOT NULL UNIQUE,
                created TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
            );
        "#)
            .execute(self)
            .await?;

        let _ = sql_query(r#"
            CREATE TABLE IF NOT EXISTS names (
                player_id INTEGER NOT NULL,
                name VARCHAR(32) NOT NULL,
                last_used TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
                PRIMARY KEY (player_id, name),
                FOREIGN KEY (player_id) REFERENCES players(internal_id)
            );
        "#)
            .execute(self)
            .await?;

        let _ = sql_query(r#"
            CREATE TABLE IF NOT EXISTS match_players (
                match_id INTEGER NOT NULL,
                player_id INTEGER NOT NULL,
                PRIMARY KEY (match_id, player_id),
                FOREIGN KEY (match_id) REFERENCES matches(internal_id),
                FOREIGN KEY (player_id) REFERENCES players(internal_id)
            );
        "#)
            .execute(self)
            .await?;

        // TODO: create ranks table

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn get_in_memory_connection() -> AsyncSqliteConnection {
        let conn_res = AsyncSqliteConnection::establish(":memory:").await;

        assert!(
            conn_res.is_ok(),
            "could not establish connection to in memory database! ({})",
            conn_res.err().unwrap()
        );

        conn_res.unwrap()
    }

    async fn setup_test() -> AsyncSqliteConnection {
        let mut conn = get_in_memory_connection().await;
        let tables_res = conn.create_tables().await;

        assert!(
            tables_res.is_ok(),
            "could not create tables! ({})",
            tables_res.err().unwrap()
        );

        conn
    }

    #[tokio::test]
    async fn create_tables() {
        let conn = setup_test().await;

        // TODO: test actual table schemas
    }

    #[tokio::test]
    async fn add_player_id() {
        let mut conn = setup_test().await;
        
        let player_id = PlayerId::from("add_test");
        let added_player_res = conn.add_player_id(&player_id).await;
        assert!(
            added_player_res.is_ok(),
            "could not add player! ({})",
            added_player_res.err().unwrap()
        );

        let added_player = added_player_res.unwrap();
        assert_eq!(*player_id, *added_player.slap_id);
    }

    #[tokio::test]
    async fn add_duplicate_player_id() {
        let mut conn = setup_test().await;

        let player_id = PlayerId::from("duplicate_test");
        let added_player = conn.add_player_id(&player_id)
            .await
            .unwrap();

        let duplicate_res = conn.add_player_id(&player_id).await;
        assert!(
            duplicate_res.is_err(),
            "duplicate player added! (first: {:?}, second: {:?})",
            added_player,
            duplicate_res.ok().unwrap()
        );
    }

    #[tokio::test]
    async fn player_id_exists() {
        let mut conn = setup_test().await;

        let player_id = PlayerId::from("exists_test");
        let added_player = conn.add_player_id(&player_id)
            .await
            .unwrap();

        let retrieved_player_res = conn.get_player(&player_id).await;
        assert!(
            retrieved_player_res.is_ok(),
            "player '{:?}' does not exist! ({})",
            player_id,
            retrieved_player_res.err().unwrap()
        );

        let retrieved_player = retrieved_player_res.unwrap();
        assert_eq!(added_player, retrieved_player);
    }

    #[tokio::test]
    async fn player_id_dne() {
        let mut conn = setup_test().await;

        let player_id = PlayerId::from("dne_test");
        let retrieved_player_res = conn.get_player(&player_id).await;
        assert!(
            retrieved_player_res.is_err(),
            "player '{:?}' exists! ({:?})",
            player_id,
            retrieved_player_res.ok().unwrap()
        );
    }

    #[tokio::test]
    async fn add_one_player_name() {
        todo!();
    }

    #[tokio::test]
    async fn add_multiple_player_names() {
        todo!();
    }

    #[tokio::test]
    async fn update_last_used() {
        todo!();
    }

    #[tokio::test]
    async fn get_player_name() {
        todo!();
    }

    #[tokio::test]
    async fn get_player_names() {
        todo!();
    }

    #[tokio::test]
    async fn get_all_players_name() {
        todo!();
    }

    #[tokio::test]
    async fn get_all_players_names() {
        todo!();
    }
}
