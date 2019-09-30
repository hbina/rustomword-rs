use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use failure::Error as FError;
use r2d2;
use r2d2_postgres;
use serde_derive::{Deserialize, Serialize};

pub type Pool = r2d2::Pool<r2d2_postgres::PostgresConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager>;

pub fn execute(pool: &Pool) -> Vec<Entry> {
    let pool = pool.clone();
    match pool.get() {
        Ok(ok) => refill(ok).unwrap_or(Vec::new()),
        Err(err) => Vec::new(),
    }
}

fn refill(conn: Connection) -> Result<Vec<Entry>, FError> {
    let stmt = "SELECT * FROM entries ORDER BY RANDOM() LIMIT 100000;";

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

impl Responder for Entry {
    type Error = Error;
    type Future = Result<HttpResponse, Error>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self)?;

        // Create response and set content type
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
    }
}
