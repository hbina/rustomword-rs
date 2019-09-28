use actix_web::{web, Error as AWError};
use failure::Error;
use futures::Future;
use r2d2;
use r2d2_postgres;
use serde_derive::{Deserialize, Serialize};

pub type Pool = r2d2::Pool<r2d2_postgres::PostgresConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager>;

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

    let prep_stmt = conn.prepare(stmt).unwrap();
    for row in &prep_stmt.query(&[]).unwrap() {
        let a = Entry {
            word: row.get(0),
            wordtype: row.get(1),
            definition: row.get(2),
        };
        let b = vec![a];
        return Ok(b);
    }
    let a = Entry {
        word: "Please".to_string(),
        wordtype: "Help".to_string(),
        definition: "Me".to_string(),
    };
    let b = vec![a];
    return Ok(b);
}
