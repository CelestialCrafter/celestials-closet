pub mod args;
pub mod assets;
pub mod database;
pub mod routes;

use std::{
    fs::{File, OpenOptions},
    io::Seek,
    net::SocketAddr,
    sync::OnceLock,
    time::Duration,
};

use eyre::Result;
use log::{info, warn};
use tokio::task;

use args::ARGS;
use database::Database;

fn save(db: &Database, file: &mut File) -> Result<()> {
    file.set_len(0)?;
    file.rewind()?;
    db.save(file)?;

    Ok(())
}

async fn save_loop(db: &Database, mut file: File) {
    loop {
        match save(db, &mut file) {
            Ok(_) => info!("saved database"),
            Err(err) => warn!("could not save database: {}", err),
        }

        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}

static DATABASE: OnceLock<Database> = OnceLock::new();

#[tokio::main]
async fn main() {
    pretty_env_logger::formatted_builder()
        .parse_filters(ARGS.log.as_str())
        .init();

    // database
    let db_file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open("database.db")
        .expect("database file should open");

    let _ = DATABASE.set(Database::default());
    let db = DATABASE.get().unwrap();
    db.load(&db_file).expect("database file should load");
    task::spawn(async { save_loop(db, db_file).await });

    // serve
    let host = SocketAddr::from(([0, 0, 0, 0], ARGS.port));
    warp::serve(routes::routes()).run(host).await;
}
