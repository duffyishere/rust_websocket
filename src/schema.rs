// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "name", schema = "pg_catalog"))]
    pub struct Name;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Name;

    account (id) {
        id -> Int4,
        email -> Varchar,
        password -> Text,
        name -> Name,
    }
}
