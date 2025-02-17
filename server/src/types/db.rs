use std::borrow::Cow;
use diesel::sqlite::Sqlite;
use diesel::{Insertable, Queryable, Selectable};
use crate::db::schema::{players, matches, names, match_players};

#[derive(Insertable)]
#[diesel(table_name = players)]
#[diesel(check_for_backend(Sqlite))]
pub struct NewPlayer<'a> {
    pub slap_id: Cow<'a, str>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = players)]
#[diesel(check_for_backend(Sqlite))]
pub struct Player<'a> {
    pub internal_id: i32,
    pub slap_id: Cow<'a, str>,
}

#[derive(Insertable)]
#[diesel(table_name = matches)]
#[diesel(check_for_backend(Sqlite))]
pub struct NewMatch<'a> {
    pub match_id: Cow<'a, str>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = matches)]
#[diesel(check_for_backend(Sqlite))]
pub struct Match<'a> {
    pub internal_id: i32,
    pub match_id: Cow<'a, str>,
}

#[derive(Insertable, Queryable, Selectable)]
#[diesel(table_name = names)]
#[diesel(check_for_backend(Sqlite))]
pub struct Name {
    pub player_id: i32,
    pub name: String,
}

#[derive(Insertable, Queryable, Selectable)]
#[diesel(table_name = match_players)]
#[diesel(check_for_backend(Sqlite))]
pub struct MatchPlayer {
    pub player_id: i32,
    pub match_id: i32,
}
