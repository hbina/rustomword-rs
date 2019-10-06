use actix_web::{web, Error as AWError};
use failure::Error as FError;
use futures::Future;
use r2d2;
use r2d2_postgres;
use serde_derive::{Deserialize, Serialize};

pub type Pool = r2d2::Pool<r2d2_postgres::PostgresConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager>;

pub fn execute(pool: &Pool) -> impl Future<Item = Vec<Entry>, Error = AWError> {
    let pool = pool.clone();
    web::block(move || get_words(pool.get()?)).from_err()
}

fn get_words(conn: Connection) -> Result<Vec<Entry>, FError> {
    let stmt = "SELECT * FROM entries ORDER BY RANDOM() LIMIT 10;";

    let prep_stmt = conn.prepare(stmt).unwrap();
    prep_stmt
        .query(&[])
        .and_then(|res| {
            Ok(res
                .iter()
                .map(|row| Entry::new(row.get(0), row.get(1), row.get(2)))
                .collect())
        })
        .map_err(|err| Into::into(err))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    word: String,
    word_type: String, // Can be made an enum?
    definitions: String,
}

impl Entry {
    pub fn new(word: String, word_type: String, definitions: String) -> Entry {
        Entry {
            word: word,
            word_type: word_type,
            definitions: definitions,
        }
    }
}
