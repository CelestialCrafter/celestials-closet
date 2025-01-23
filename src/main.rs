pub mod args;
pub mod routes;

use std::net::SocketAddr;

use args::ARGS;
use warp::Filter;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let root = warp::get().then(routes::index::page);
    let routes = root.recover(routes::rejections::handle);

    let host = SocketAddr::from(([0, 0, 0, 0], ARGS.port));
    warp::serve(routes).run(host).await;
}
