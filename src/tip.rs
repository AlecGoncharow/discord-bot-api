use diesel::{
    prelude::*,
    pg::PgConnection,
};

pub fn is_valid_key(conn: &PgConnection, provided_key: i64) -> bool {
    use crate::schema::keys::dsl::*;
    let results = keys.filter(key.eq(&provided_key)).load::<crate::models::Key>(conn).expect("help");
    results.len() == 1
}