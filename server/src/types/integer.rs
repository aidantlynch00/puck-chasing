use diesel::{AsExpression, FromSqlRow};
use diesel::sql_types::Integer;
use diesel::backend::Backend;
use diesel::sqlite::Sqlite;
use diesel::serialize::{ToSql, Output, Result as SerResult};
use diesel::deserialize::{FromSql, Result as DeserResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, AsExpression, FromSqlRow)]
#[sql_type = "Integer"]
struct InternalId(i32);

impl ToSql<Integer, Sqlite> for InternalId {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> SerResult {
        <i32 as ToSql<Integer, Sqlite>>::to_sql(&self.0, out)
    }
}

impl FromSql<Integer, Sqlite> for InternalId {
    fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> DeserResult<Self> {
        let id = <i32 as FromSql<Integer, Sqlite>>::from_sql(bytes)
            .expect("column can be expressed as integer type!");

        Ok(InternalId(id))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, AsExpression, FromSqlRow)]
#[sql_type = "Integer"]
pub struct InternalPlayerId(InternalId);

impl ToSql<Integer, Sqlite> for InternalPlayerId {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> SerResult {
        <InternalId as ToSql<Integer, Sqlite>>::to_sql(&self.0, out)
    }
}

impl FromSql<Integer, Sqlite> for InternalPlayerId {
    fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> DeserResult<Self> {
        let id = <InternalId as FromSql<Integer, Sqlite>>::from_sql(bytes)
            .expect("column can be expressed as internal ID!");

        Ok(InternalPlayerId(id))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, AsExpression, FromSqlRow)]
#[sql_type = "Integer"]
pub struct InternalMatchId(InternalId);

impl ToSql<Integer, Sqlite> for InternalMatchId {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> SerResult {
        <InternalId as ToSql<Integer, Sqlite>>::to_sql(&self.0, out)
    }
}

impl FromSql<Integer, Sqlite> for InternalMatchId {
    fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> DeserResult<Self> {
        let id = <InternalId as FromSql<Integer, Sqlite>>::from_sql(bytes)
            .expect("column can be expressed as internal ID!");

        Ok(InternalMatchId(id))
    }
}
