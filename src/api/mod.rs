use actix_web::{Error as AWError, HttpResponse};
use futures::future::Future;
use tera::{Context, Tera};

use crate::db;

pub fn index(
    pool: actix_web::web::Data<db::Pool>,
    templates: actix_web::web::Data<Tera>,
) -> impl Future<Item = HttpResponse, Error = AWError> {
    db::execute(&pool).from_err().and_then(move |entries| {
        let mut context = Context::new();
        context.insert("entries", &entries);
        let rendered = templates
            .render("index.html.tera", &context)
            .map_err(|e| actix_web::error::ErrorInternalServerError(e.description().to_owned()))?;

        Ok(HttpResponse::Ok().body(rendered))
    })
}
