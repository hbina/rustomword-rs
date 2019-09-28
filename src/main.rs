use std::io;

use actix_web::{middleware, web, App, Error as AWError, HttpResponse, HttpServer};
use futures::future::Future;
use r2d2_sqlite;
use r2d2_sqlite::SqliteConnectionManager;

mod db;
use db::Pool;

fn index(db: web::Data<Pool>) -> impl Future<Item = HttpResponse, Error = AWError> {
    db::execute(&db)
        .from_err()
        .and_then(|res| Ok(HttpResponse::Ok().json(res)))
}

fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix_rt::System::new("rustom_word");

    // Start N db executor actors (N = number of cores avail)
    let manager = SqliteConnectionManager::file("dictionary.db");
    let pool = Pool::new(manager).unwrap();

    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to_async(index)))
    })
    .bind("127.0.0.1:8080")?
    .start();

    println!("Started http server: 127.0.0.1:8080");
    sys.run()
}
