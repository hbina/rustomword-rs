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
    web::block(move || get_random_word(pool.get()?)).from_err()
}

fn get_random_word(conn: Connection) -> Result<Vec<Entry>, FError> {
    let stmt = "SELECT * FROM entries ORDER BY RANDOM() LIMIT 1;";

    let prep_stmt = conn.prepare(stmt).unwrap();
    for row in &prep_stmt.query(&[]).unwrap() {
        let a = Entry::new(row.get(0), row.get(1), row.get(2));
        let b = vec![a];
        return Ok(b);
    }
    let a = Entry::new("Please".to_string(), "Help".to_string(), "Me".to_string());
    let b = vec![a];
    return Ok(b);
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
