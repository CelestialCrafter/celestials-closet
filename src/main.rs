pub mod args;
pub mod assets;
pub mod errors;
pub mod routes;

use std::net::SocketAddr;

use args::ARGS;
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
    let projects = warp::path("projects").then(routes::projects::page);
    let posts = warp::path("blog")
        .and(warp::path::end())
        .then(routes::blog::page);
    let post = posts.or(warp::path("blog")
        .and(warp::path::param())
        .and_then(routes::blog::post_page));

    let pages = index.or(projects).or(post);
    let assets = warp::path("assets")
        .and(warp::fs::dir("assets"))
        .map(|reply| {
            warp::reply::with_header(
                reply,
                "Cache-Control",
                format!("max-age={}", 60 * 60 * 24 * 7),
            )
        });

    let routes = assets
        .or(pages)
        .recover(routes::rejections::handle)
        .with(warp::filters::compression::brotli());

    // serve
    let host = SocketAddr::from(([0, 0, 0, 0], ARGS.port));
    warp::serve(routes).run(host).await;
}
