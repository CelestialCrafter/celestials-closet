pub mod args;
pub mod errors;
pub mod routes;

use std::{net::SocketAddr, sync::Arc};

use args::ARGS;
use log::error;
use warp::Filter;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let r#static = warp::path("static").and(warp::fs::dir("static"));
    let root = warp::path::end().then(routes::index::page);
    let blog = warp::path("blog")
        .and(warp::path::param())
        .and_then(routes::blog::page);

    let repositories = Arc::new(match routes::projects::repositories().await {
        Ok(r) => r,
        Err(err) => {
            error!("could not fetch repositories: {}", err);
            vec![]
        }
    });

    let projects = warp::path("projects")
        .map(move || repositories.clone())
        .then(routes::projects::page);

    let routes = root
        .or(r#static)
        .or(blog)
        .or(projects)
        .recover(routes::rejections::handle);

    let host = SocketAddr::from(([0, 0, 0, 0], ARGS.port));
    warp::serve(routes).run(host).await;
}
