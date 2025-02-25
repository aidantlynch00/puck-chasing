use std::borrow::Cow;
use time::OffsetDateTime;
use diesel::prelude::*;
use diesel::dsl::*;
use diesel::result::Error;
use diesel_async::RunQueryDsl;
use diesel_async::sync_connection_wrapper::SyncConnectionWrapper;
use diesel_async::pooled_connection::bb8::PooledConnection;
use crate::types::slapshot::{Player, PlayerId, Username};
use crate::types::db::{NewPlayerRow, PlayerRow, NewNameRow, NameRow};

pub type AsyncSqliteConnection = SyncConnectionWrapper<SqliteConnection>;

pub struct Connection<'a> {
    conn: PooledConnection<'a, AsyncSqliteConnection>,
}

impl<'a> Connection<'a> {
    pub async fn add_player(&mut self, player: &Player) -> Result<PlayerRow, Error> {
        use crate::db::schema::players::dsl::*;

        let new_player = NewPlayerRow {
            slap_id: Cow::Borrowed(&player.game_user_id),
        };

        insert_into(players)
            .values(&new_player)
            .returning(PlayerRow::as_returning())
            .get_result(&mut self.conn)
            .await
    }

    pub async fn get_player(&mut self, id: &PlayerId) -> Result<PlayerRow, Error> {
        use crate::db::schema::players::dsl::*;

        players
            .filter(slap_id.eq(Cow::<'_, str>::Borrowed(id)))
            .select(PlayerRow::as_select())
            .get_result(&mut self.conn)
            .await
    }

    pub async fn add_or_update_player_name(
        &mut self,
        player_row: &PlayerRow<'_>,
        username: Username,
    ) -> Result<NameRow, Error> {
        use crate::db::schema::names::dsl::*;

        let new_name = NewNameRow {
            player_id: player_row.internal_id,
            name: Cow::Borrowed(&username),
        };

        // insert the new name or update the last used timestamp if it exists
        insert_into(names)
            .values(&new_name)
            .on_conflict((player_id, name))
            .do_update()
            .set(last_used.eq(OffsetDateTime::now_utc()))
            .returning(NameRow::as_returning())
            .get_result(&mut self.conn)
            .await
    }

    pub async fn get_player_names(
        &mut self,
        player_row: &PlayerRow<'_>
    ) -> Result<Vec<NameRow<'_>>, Error> {
        NameRow::belonging_to(player_row)
            .select(NameRow::as_select())
            .load(&mut self.conn)
            .await
    }

    pub async fn get_all_player_names(
        &mut self,
    ) -> Result<Vec<(PlayerRow, Vec<NameRow>)>, Error> {
        use crate::db::schema::players;

        let all_players = players::table
            .select(PlayerRow::as_select())
            .load(&mut self.conn)
            .await?;

        let all_names = NameRow::belonging_to(&all_players)
            .select(NameRow::as_select())
            .load(&mut self.conn)
            .await?;

        let player_names = all_names
            .grouped_by(&all_players)
            .into_iter()
            .zip(all_players)
            .map(|(names, player)| (player, names))
            .collect::<Vec<(PlayerRow, Vec<NameRow>)>>();

        Ok(player_names)
    }

    pub async fn create_tables(&mut self) -> Result<(), Error> {
        let _ = sql_query(r#"
            CREATE TABLE IF NOT EXISTS players (
                internal_id INTEGER PRIMARY KEY AUTOINCREMENT,
                slap_id VARCHAR(40) NOT NULL UNIQUE
            );
        "#)
            .execute(&mut self.conn)
            .await?;

        // TODO: add a timestamp
        let _ = sql_query(r#"
            CREATE TABLE IF NOT EXISTS matches (
                internal_id INTEGER PRIMARY KEY AUTOINCREMENT,
                match_id VAR_CHAR(40) NOT NULL UNIQUE
                created TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
            );
        "#)
            .execute(&mut self.conn)
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
            .execute(&mut self.conn)
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
            .execute(&mut self.conn)
            .await?;

        // TODO: create ranks table

        Ok(())
    }
}

impl<'a> From<PooledConnection<'a, AsyncSqliteConnection>> for Connection<'a> {
    fn from(conn: PooledConnection<'a, AsyncSqliteConnection>) -> Connection<'a> {
        Connection { conn }
    }
}
