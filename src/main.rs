#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod db;
mod entry;
mod error;
mod schema;

use crate::entry::{Entry, EntryProxy};
use crate::error::ApiError;
use actix_web::{get, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;

#[get("/")]
async fn index() -> Result<HttpResponse, ApiError> {
    let entry_proxy = EntryProxy::find_random()?;
    Ok(HttpResponse::Ok().json(Entry::from(entry_proxy)))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    db::init();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| App::new().service(index));

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Host not set");
            let port = env::var("PORT").expect("Port not set");
            server.bind(format!("{}:{}", host, port))?
        }
    };
    server.run().await
}
