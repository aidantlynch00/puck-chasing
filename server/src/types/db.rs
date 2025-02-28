use time::OffsetDateTime;
use diesel::sqlite::Sqlite;
use diesel::{Insertable, Queryable, Selectable, Identifiable, Associations};

use crate::db::schema::{players, matches, names, match_players};
use crate::types::string::*;
use crate::types::integer::*;

#[derive(Insertable)]
#[diesel(table_name = players)]
#[diesel(check_for_backend(Sqlite))]
pub struct NewPlayerRow {
    pub slap_id: PlayerId,
}

#[derive(Debug, PartialEq, Eq, Queryable, Selectable, Identifiable)]
#[diesel(table_name = players)]
#[diesel(primary_key(internal_id))]
#[diesel(check_for_backend(Sqlite))]
pub struct PlayerRow {
    pub internal_id: InternalPlayerId,
    pub slap_id: PlayerId,
}

#[derive(Insertable)]
#[diesel(table_name = matches)]
#[diesel(check_for_backend(Sqlite))]
pub struct NewMatchRow {
    pub match_id: MatchId,
    pub created: OffsetDateTime,
}

#[derive(Debug, PartialEq, Eq, Queryable, Selectable, Identifiable)]
#[diesel(table_name = matches)]
#[diesel(primary_key(internal_id))]
#[diesel(check_for_backend(Sqlite))]
pub struct MatchRow {
    pub internal_id: InternalMatchId,
    pub match_id: MatchId,
    pub created: OffsetDateTime,
}


#[derive(Insertable)]
#[diesel(table_name = names)]
#[diesel(check_for_backend(Sqlite))]
pub struct NewNameRow {
    pub player_id: InternalPlayerId,
    pub name: Username,
}

#[derive(Debug, PartialEq, Eq, Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name = names)]
#[diesel(primary_key(player_id, name))]
#[diesel(belongs_to(PlayerRow, foreign_key = player_id))]
#[diesel(check_for_backend(Sqlite))]
pub struct NameRow {
    pub player_id: InternalPlayerId,
    pub name: Username,
    pub last_used: OffsetDateTime,
}

#[derive(Debug, PartialEq, Eq, Insertable, Queryable, Selectable, Identifiable)]
#[diesel(table_name = match_players)]
#[diesel(primary_key(player_id, match_id))]
#[diesel(check_for_backend(Sqlite))]
pub struct MatchPlayerRow {
    pub player_id: InternalPlayerId,
    pub match_id: InternalMatchId,
}
