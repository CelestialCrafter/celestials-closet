pub mod args;
pub mod assets;
pub mod errors;
pub mod routes;

use std::{net::SocketAddr, sync::Arc};

use args::ARGS;
use log::error;
use warp::Filter;

fn logging() {
    let mut builder = pretty_env_logger::formatted_builder();
    match &ARGS.log {
        Some(filter) => builder.parse_filters(filter.as_str()),
        None => &mut builder,
    }
    .init();
}

#[tokio::main]
async fn main() {
    logging();

    let index = warp::path::end().then(routes::index::page);
    let blog = warp::path("blog")
        .and(warp::path::param())
        .and_then(routes::blog::page);

    let projects = {
        let repositories = Arc::new(match routes::projects::repositories().await {
            Ok(r) => r,
            Err(err) => {
                error!("could not fetch repositories: {}", err);
                vec![]
            }
        });

        warp::path("projects")
            .map(move || repositories.clone())
            .then(routes::projects::page)
    };

    let pages = index.or(blog).or(projects);
    let assets = warp::path("assets")
        .and(warp::fs::dir("assets"))
        .map(|reply| {
            warp::reply::with_header(
                reply,
                "Cache-Control",
                format!("max-age={}", 60 * 60 * 24 * 7),
            )
        });

    let routes = assets.or(pages).recover(routes::rejections::handle);

    // serve
    let host = SocketAddr::from(([0, 0, 0, 0], ARGS.port));
    warp::serve(routes).run(host).await;
}
