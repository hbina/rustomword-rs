use actix_web::Responder;
use rand::seq::SliceRandom;
use std::sync::{Arc, Mutex};

use crate::db;

pub fn index(data: actix_web::web::Data<Arc<Mutex<Vec<db::Entry>>>>) -> impl Responder {
    Arc::try_unwrap(*data)
        .and_then(|res| {
            let r = res.lock().unwrap();
            let choices = *r;
            choices[0]
        })
        .and_then(|ok| ok);
}
