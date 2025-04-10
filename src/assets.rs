use warp::{
    filters::{
        fs::dir,
        path::{path, Tail},
        BoxedFilter,
    },
    reject,
    reply::Reply,
    Filter,
};

include!(concat!(env!("OUT_DIR"), "/assets.rs"));

pub fn route() -> BoxedFilter<(impl Reply,)> {
    path("assets")
        .and(dir("assets"))
        .map(|reply| {
            warp::reply::with_header(
                reply,
                "Cache-Control",
                format!("max-age={}", 60 * 60 * 24 * 30),
            )
        })
        .boxed()
}

pub async fn page(tail: Tail) -> Result<impl Reply, reject::Rejection> {
    match ASSETS.get(tail.as_str()) {
        Some(data) => Ok(*data),
        None => Err(reject::not_found()),
    }
}
