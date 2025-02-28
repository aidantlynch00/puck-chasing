use std::sync::Arc;
use std::ops::Deref;
use serde::Deserialize;
use diesel::{AsExpression, FromSqlRow};
use diesel::sql_types::Text;
use diesel::backend::Backend;
use diesel::sqlite::Sqlite;
use diesel::serialize::{ToSql, Output, Result as SerResult};
use diesel::deserialize::{FromSql, Result as DeserResult};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, AsExpression, FromSqlRow)]
#[sql_type = "Text"]
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

impl ToSql<Text, Sqlite> for Username {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> SerResult {
        <str as ToSql<Text, Sqlite>>::to_sql(self.deref(), out)
    }
}

impl FromSql<Text, Sqlite> for Username {
    fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> DeserResult<Self> {
        let owned = <String as FromSql<Text, Sqlite>>::from_sql(bytes)
            .expect("column can be expressed as string type!");

        Ok(Username::from(owned.as_str()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, AsExpression, FromSqlRow)]
#[sql_type = "Text"]
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

impl ToSql<Text, Sqlite> for PlayerId {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> SerResult {
        <str as ToSql<Text, Sqlite>>::to_sql(self.deref(), out)
    }
}

impl FromSql<Text, Sqlite> for PlayerId {
    fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> DeserResult<Self> {
        let owned = <String as FromSql<Text, Sqlite>>::from_sql(bytes)
            .expect("column can be expressed as string type!");

        Ok(PlayerId::from(owned.as_str()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, AsExpression, FromSqlRow)]
#[sql_type = "Text"]
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

impl ToSql<Text, Sqlite> for MatchId {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> SerResult {
        <str as ToSql<Text, Sqlite>>::to_sql(self.deref(), out)
    }
}

impl FromSql<Text, Sqlite> for MatchId {
    fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> DeserResult<Self> {
        let owned = <String as FromSql<Text, Sqlite>>::from_sql(bytes)
            .expect("column can be expressed as string type!");

        Ok(MatchId::from(owned.as_str()))
    }
}
