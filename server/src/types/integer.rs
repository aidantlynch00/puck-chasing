macro_rules! integer_type {
    ($type:ident) => {
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, Hash,
            ::diesel::AsExpression, ::diesel::FromSqlRow,
        )]
        #[diesel(sql_type = ::diesel::sql_types::Integer)]
        pub struct $type(i32);

        impl ::diesel::serialize::ToSql<::diesel::sql_types::Integer, ::diesel::sqlite::Sqlite> for $type {
            fn to_sql<'b>(
                &'b self,
                out: &mut ::diesel::serialize::Output<'b, '_, ::diesel::sqlite::Sqlite>
            ) -> ::diesel::serialize::Result {
                <i32 as ::diesel::serialize::ToSql<::diesel::sql_types::Integer, ::diesel::sqlite::Sqlite>>::to_sql(
                    &self.0,
                    out
                )
            }
        }

        impl ::diesel::deserialize::FromSql<::diesel::sql_types::Integer, ::diesel::sqlite::Sqlite> for $type {
            fn from_sql(
                bytes: <::diesel::sqlite::Sqlite as ::diesel::backend::Backend>::RawValue<'_>
            ) -> ::diesel::deserialize::Result<Self> {
                let id = <i32 as ::diesel::deserialize::FromSql<::diesel::sql_types::Integer, ::diesel::sqlite::Sqlite>>::from_sql(bytes)
                    .expect("column can be expressed as integer type!");

                Ok($type(id))
            }
        }
    }
}

integer_type!(InternalPlayerId);
integer_type!(InternalMatchId);
