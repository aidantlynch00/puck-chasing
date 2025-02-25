use std::borrow::Cow;
use diesel::sqlite::Sqlite;
use diesel::{Insertable, Queryable, Selectable};
use crate::db::schema::{players, matches, names, match_players};

#[derive(Insertable)]
#[diesel(table_name = players)]
#[diesel(check_for_backend(Sqlite))]
pub struct NewPlayerRow<'a> {
    pub slap_id: Cow<'a, str>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = players)]
#[diesel(check_for_backend(Sqlite))]
pub struct PlayerRow<'a> {
    pub internal_id: i32,
    pub slap_id: Cow<'a, str>,
}

#[derive(Insertable)]
#[diesel(table_name = matches)]
#[diesel(check_for_backend(Sqlite))]
pub struct NewMatchRow<'a> {
    pub match_id: Cow<'a, str>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = matches)]
#[diesel(check_for_backend(Sqlite))]
pub struct MatchRow<'a> {
    pub internal_id: i32,
    pub match_id: Cow<'a, str>,
}

#[derive(Insertable, Queryable, Selectable)]
#[diesel(table_name = names)]
#[diesel(check_for_backend(Sqlite))]
pub struct NameRow<'a> {
    pub player_id: i32,
    pub name: Cow<'a, str>,
}

#[derive(Insertable, Queryable, Selectable)]
#[diesel(table_name = match_players)]
#[diesel(check_for_backend(Sqlite))]
pub struct MatchPlayerRow {
    pub player_id: i32,
    pub match_id: i32,
}
