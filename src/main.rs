pub mod args;
pub mod assets;
pub mod database;
pub mod routes;

use std::{
    fs::OpenOptions,
    io::{Seek, SeekFrom},
    net::SocketAddr,
    sync::Arc,
    time::Duration,
};

use args::ARGS;
use database::Database;
use log::warn;
use tokio::task;

#[tokio::main]
async fn main() {
    // logging
    let mut builder = pretty_env_logger::formatted_builder();
    match &ARGS.log {
        Some(filter) => builder.parse_filters(filter.as_str()),
        None => &mut builder,
    }
    .init();

    // database
    let mut db_file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open("database.db")
        .expect("should be able to open database file");
    let db = Arc::new(Database::new(&mut db_file).expect("should be able to create database"));

    {
        let db = db.clone();
        task::spawn(async move {
            loop {
                if let Err(err) = db_file
                    .seek(SeekFrom::Start(0))
                    .and_then(|_| Ok(db.save(&mut db_file)))
                {
                    warn!("could not save database: {err}")
                }

                tokio::time::sleep(Duration::from_secs(10)).await;
            }
        });
    }

    // serve
    let host = SocketAddr::from(([0, 0, 0, 0], ARGS.port));
    warp::serve(routes::routes(db)).run(host).await;
}
