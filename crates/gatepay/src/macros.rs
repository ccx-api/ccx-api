#[cfg(feature = "with_diesel")]
macro_rules! impl_diesel1 {
    ($type_name:ty) => {
        mod with_diesel1 {
            use std::io::Write;
            use std::str::FromStr;

            use diesel::deserialize::FromSql;
            use diesel::serialize::ToSql;
            use diesel::sql_types::Text;

            use super::*;

            impl<DB> ToSql<Text, DB> for $type_name
            where
                DB: diesel::backend::Backend,
                str: ToSql<Text, DB>,
            {
                fn to_sql<W: Write>(
                    &self,
                    out: &mut diesel::serialize::Output<W, DB>,
                ) -> diesel::serialize::Result {
                    self.as_ref().to_sql(out)
                }
            }

            impl<DB> FromSql<Text, DB> for $type_name
            where
                DB: diesel::backend::Backend,
                String: FromSql<Text, DB>,
            {
                fn from_sql(bytes: Option<&DB::RawValue>) -> diesel::deserialize::Result<Self> {
                    let name = String::from_sql(bytes)?;
                    Ok(Self::from_str(name.as_str())?)
                }
            }
        }
    };
}
