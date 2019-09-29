use actix_web::{Error as AWError, HttpResponse};
use futures::future::Future;

use crate::db;

pub fn index(
    db: actix_web::web::Data<db::Pool>,
) -> impl Future<Item = HttpResponse, Error = AWError> {
    db::execute(&db)
        .from_err()
        .and_then(|res| Ok(HttpResponse::Ok().json(res)))
}
