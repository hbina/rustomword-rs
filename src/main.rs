use actix_web::{middleware, web, App, HttpServer};
use r2d2_postgres;

mod api;
mod db;

fn main() -> std::io::Result<()> {
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

    // Start N db executor actors (N = number of cores avail)
    let manager =
        r2d2_postgres::PostgresConnectionManager::new(database_url, r2d2_postgres::TlsMode::None)
            .map_err(|error| println!("unable to connect to error:{}", error))
            .unwrap();
    let pool = db::Pool::new(manager).unwrap();
    // let words_pool = std::sync::Mutex::new(;
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to_async(api::index)))
    })
    .bind(("0.0.0.0", port))
    .map_err(|error| {
        panic!(
            "Unable to bind to port 0.0.0.0/{} with error:{}",
            port, error
        )
    })
    .map(|res| {
        println!("launching server");
        res.run().unwrap()
    })
}
