macro_rules! string_type {
    ($type:ident) => {
        #[derive(
            Debug, Clone, PartialEq, Eq, Hash,
            ::serde::Serialize, ::serde::Deserialize,
            ::diesel::AsExpression, ::diesel::FromSqlRow,
        )]
        #[diesel(sql_type = ::diesel::sql_types::Text)]
        pub struct $type(::std::sync::Arc<str>);

        impl From<&str> for $type {
            fn from(value: &str) -> Self {
                $type(::std::sync::Arc::from(value))
            }
        }

        impl ::std::ops::Deref for $type {
            type Target = str;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl ::std::fmt::Display for $type {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str(self)
            }
        }

        impl ::diesel::serialize::ToSql<::diesel::sql_types::Text, ::diesel::sqlite::Sqlite> for $type {
            fn to_sql<'b>(
                &'b self,
                out: &mut ::diesel::serialize::Output<'b, '_, ::diesel::sqlite::Sqlite>
            ) -> ::diesel::serialize::Result {
                <str as ::diesel::serialize::ToSql<::diesel::sql_types::Text, ::diesel::sqlite::Sqlite>>::to_sql(
                    <Self as ::std::ops::Deref>::deref(self),
                    out
                )
            }
        }

        impl ::diesel::deserialize::FromSql<::diesel::sql_types::Text, ::diesel::sqlite::Sqlite> for $type {
            fn from_sql(
                bytes: <::diesel::sqlite::Sqlite as ::diesel::backend::Backend>::RawValue<'_>
            ) -> ::diesel::deserialize::Result<Self> {
                let owned = <String as ::diesel::deserialize::FromSql<::diesel::sql_types::Text, ::diesel::sqlite::Sqlite>>::from_sql(bytes)
                    .expect("column can be expressed as string type!");

                Ok($type::from(owned.as_str()))
            }
        }
    };
}

string_type!(Username);
string_type!(PlayerId);
string_type!(MatchId);
