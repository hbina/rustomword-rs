use actix_web::{middleware, web, App, HttpServer};
use r2d2_postgres;
use std::sync::{Arc, Mutex};
use std::time::Duration;

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
    let local_data: Arc<Mutex<Vec<db::Entry>>> = Arc::new(Mutex::new(db::execute(&pool)));
    // CRON job to refresh random words
    let h1 = std::thread::spawn(move || loop {
        println!("running CRON job...");
        std::thread::sleep(Duration::from_secs(5));
    });

    let h2 = std::thread::spawn(move || {
        let data_proxy = web::Data::new(local_data);
        match HttpServer::new(move || {
            App::new()
                .data(data_proxy.clone())
                .wrap(middleware::Logger::default())
                .service(web::resource("/").route(web::get().to_async(api::index)))
        })
        .bind(("0.0.0.0", port.clone()))
        {
            Ok(ok) => ok.run().unwrap(),
            Err(err) => panic!("unable to start server err:{}", err),
        }
    });
    h2.join().unwrap();
    h1.join().unwrap();
    Ok(())
}
