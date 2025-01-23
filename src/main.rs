pub mod args;
pub mod errors;
pub mod routes;

use std::net::SocketAddr;

use args::ARGS;
use warp::Filter;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let root = warp::path::end().then(routes::index::page);
    let r#static = warp::path("static").and(warp::fs::dir("static"));
    let blog = warp::path("blog")
        .and(warp::path::param())
        .and_then(routes::blog::page);

    let routes = root
        .or(r#static)
        .or(blog)
        .recover(routes::rejections::handle);

    let host = SocketAddr::from(([0, 0, 0, 0], ARGS.port));
    warp::serve(routes).run(host).await;
}
