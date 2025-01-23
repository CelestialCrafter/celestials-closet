use args::ARGS;
use warp::Filter;

pub mod args;

#[tokio::main]
async fn main() {
    let root = warp::path::end().map(|| "hai :3");

    warp::serve(root).run(([0, 0, 0, 0], ARGS.port)).await;
}
