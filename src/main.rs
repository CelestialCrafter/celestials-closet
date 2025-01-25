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

    // special
    let r#static = warp::path("static").and(warp::fs::dir("static"));
    let base = warp::path("base.css").map(|| {
        warp::reply::with_header(
            grass::include!("styles/base.scss"),
            "Content-Type",
            "text/css",
        )
    });
    let oneko = warp::path("oneko.css").map(|| {
        warp::reply::with_header(
            grass::include!("styles/oneko.scss"),
            "Content-Type",
            "text/css",
        )
    });

    let special = r#static.or(base).or(oneko);

    // pages
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

    // serve
    let routes = special.or(pages).recover(routes::rejections::handle);
    let host = SocketAddr::from(([0, 0, 0, 0], ARGS.port));
    warp::serve(routes).run(host).await;
}
