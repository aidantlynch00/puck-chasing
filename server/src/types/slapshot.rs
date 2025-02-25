use std::ops::Deref;
use std::sync::Arc;
use serde::Deserialize;
use serde::de::Error;
use time::OffsetDateTime;
use time::serde::iso8601;

#[derive(Clone, Deserialize)]
pub struct Username(Arc<str>);

impl From<&str> for Username {
    fn from(value: &str) -> Self {
        Username(Arc::from(value))
    }
}

impl Deref for Username {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct PlayerId(Arc<str>);

impl From<&str> for PlayerId {
    fn from(value: &str) -> Self {
        PlayerId(Arc::from(value))
    }
}

impl Deref for PlayerId {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Deserialize)]
pub struct MatchId(Arc<str>);

impl From<&str> for MatchId {
    fn from(value: &str) -> Self {
        MatchId(Arc::from(value))
    }
}

impl Deref for MatchId {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Deserialize)]
pub struct RecentHistory {
    #[serde(flatten)]
    pub player: Player,
    pub match_history: Vec<Match>,
}

#[derive(Deserialize)]
pub struct Player {
    pub username: Username,
    pub game_user_id: PlayerId,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Hockey,
    Pond,
    Dodgepuck,
    Tag,
}

#[derive(Deserialize)]
pub struct Match {
    pub id: MatchId,
    #[serde(with = "iso8601")]
    pub created: OffsetDateTime,
    #[serde(rename = "gamemode")]
    pub mode: Mode,
    pub game_stats: Option<MatchStats>,
}

#[derive(Deserialize)]
pub struct MatchStats {
    #[serde(deserialize_with = "none_as_option")]
    pub winner: Option<Team>,
    pub score: Score,
    pub players: Vec<MatchPlayer>
}

#[derive(Deserialize)]
pub struct Score {
    pub away: u8,
    pub home: u8
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Team {
    Home,
    Away
}

fn none_as_option<'de, D>(deserializer: D) -> Result<Option<Team>, D::Error>
where D: serde::Deserializer<'de>
{
    let slice: &str = Deserialize::deserialize(deserializer)?;
    match slice {
        "none" => Ok(None),
        "home" => Ok(Some(Team::Home)),
        "away" => Ok(Some(Team::Away)),
        _ => Err(Error::custom(format!("Invalid team: {}", slice)))
    }
}

#[derive(Deserialize)]
pub struct MatchPlayer {
    #[serde(flatten)]
    pub player: Player,
    pub team: Team,
    pub stats: PlayerStats
}

#[derive(Deserialize)]
pub struct PlayerStats {}
