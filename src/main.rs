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
        .unwrap_or_else(|err| {
            println!("parsing $PORT returns err:{} defaulting to 3000", err);
            "3000".to_string()
        })
        .parse()
        .expect("PORT must be a number");

    let database_url: String = std::env::var("HEROKU_POSTGRESQL_CYAN_URL")
        .unwrap_or_else(|err| {
            println!(
                "parsing $HEROKU_POSTGRESQL_CYAN_URL returns err:{} defaulting to localhost:4532",
                err
            );
            "postgres://rustomword_test:654321@localhost:5432/rustomword_test".to_string()
        })
        .parse()
        .expect("Invalid database URI");

    env_logger::init();
    let sys = actix_rt::System::new("rustom_word");

    // Start N db executor actors (N = number of cores avail)
    let manager =
        r2d2_postgres::PostgresConnectionManager::new(database_url, r2d2_postgres::TlsMode::None)
            .map_err(|error| println!("unable to connect to error:{}", error))
            .unwrap();
    let pool = Pool::new(manager).unwrap();

    // Start http server
    match HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to_async(index)))
    })
    .bind(("0.0.0.0", port))
    {
        Ok(ok) => {
            println!("launching server"); // TODO :: Print current time
            ok.start();
        }
        Err(err) => {
            panic!("unable to bind:{}", err);
        }
    };
    sys.run()
}
