use std::io;

use actix_web::{middleware, web, App, Error as AWError, HttpResponse, HttpServer};
use futures::future::Future;
use r2d2_postgres;

mod db;
use db::Pool;

fn index(db: web::Data<Pool>) -> impl Future<Item = HttpResponse, Error = AWError> {
    db::execute(&db)
        .from_err()
        .and_then(|res| Ok(HttpResponse::Ok().json(res)))
}

fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    let database_url: String = std::env::var("HEROKU_POSTGRESQL_CYAN_URL")
        .unwrap_or_else(|_| "localhost:4532".to_string())
        .parse()
        .expect("Invalid database URL");

    env_logger::init();
    let sys = actix_rt::System::new("rustom_word");

    // Start N db executor actors (N = number of cores avail)
    let manager =
        r2d2_postgres::PostgresConnectionManager::new(database_url, r2d2_postgres::TlsMode::None)
            .unwrap();
    let pool = Pool::new(manager).unwrap();

    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to_async(index)))
    })
    .bind(("0.0.0.0", port))?
    .start();
    sys.run()
}
