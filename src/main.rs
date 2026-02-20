pub mod args;
pub mod assets;
pub mod routes;

use std::net::SocketAddr;

use args::ARGS;
use log::info;

#[tokio::main]
async fn main() {
    pretty_env_logger::formatted_builder()
        .parse_filters(ARGS.log.as_str())
        .init();

    // serve
    let host = SocketAddr::from(([0, 0, 0, 0], ARGS.port));
    info!("listening on {host}");
    warp::serve(routes::routes()).run(host).await;
}
