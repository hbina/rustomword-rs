use actix_web::{web, Error as AWError};
use failure::Error;
use futures::Future;
use r2d2;
use r2d2_sqlite;
use rusqlite::NO_PARAMS;
use serde_derive::{Deserialize, Serialize};

pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    word: String,
    wordtype: String, // Can be made an enum?
    definition: String,
}

pub fn execute(pool: &Pool) -> impl Future<Item = Vec<Entry>, Error = AWError> {
    let pool = pool.clone();
    web::block(move || get_random_word(pool.get()?)).from_err()
}

fn get_random_word(conn: Connection) -> Result<Vec<Entry>, Error> {
    let stmt = "SELECT * FROM entries ORDER BY RANDOM() LIMIT 1;";

    let mut prep_stmt = conn.prepare(stmt)?;
    let annuals = prep_stmt
        .query_map(NO_PARAMS, |row| Entry {
            word: row.get(0),
            wordtype: row.get(1),
            definition: row.get(2),
        })
        .and_then(|mapped_rows| Ok(mapped_rows.map(|row| row.unwrap()).collect::<Vec<Entry>>()))?;
    Ok(annuals)
}
