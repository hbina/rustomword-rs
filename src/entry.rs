use crate::db;
use crate::error::ApiError;
use crate::schema::entries;
use crate::schema::entries::columns::id;

use diesel::prelude::*;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Queryable, Insertable, Deserialize)]
#[table_name = "entries"]
pub struct EntryProxy {
    pub id: i32,
    pub word: String,
    pub wordtype: String,
    pub definition: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub enum WordType {
    Noun,
    Adverb,
    Adjective,
    Plural,
    Unknown,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Entry {
    pub id: u32,
    pub word: String,
    pub word_type: WordType,
    pub definition: Vec<String>,
}

impl From<EntryProxy> for Entry {
    fn from(value: EntryProxy) -> Self {
        let word_type = match value.wordtype.as_str() {
            "n." => WordType::Noun,
            "adv." => WordType::Adverb,
            "a." => WordType::Adjective,
            "pl." => WordType::Plural,
            _ => WordType::Unknown,
        };
        Entry {
            id: value.id as u32,
            word: value.word,
            word_type,
            definition: value
                .definition
                .split(';')
                .map(|x| x.trim_start())
                // .map(String::from)
                .map(|x| x.replace("\n   ", " "))
                .collect::<Vec<_>>(),
        }
    }
}

impl EntryProxy {
    pub fn find_random() -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let mut rng = rand::thread_rng();
        let key: i32 = rng.gen_range(1, 176023);

        let proxy = entries::table
            .filter(id.eq(key))
            .first::<EntryProxy>(&conn)?;

        Ok(proxy)
    }
}
