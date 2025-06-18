use warp::{
    filters::{
        path::{path, tail, Tail},
        BoxedFilter,
    },
    reject, reply, Filter,
};

include!(concat!(env!("OUT_DIR"), "/assets.rs"));

pub fn route() -> BoxedFilter<(impl reply::Reply,)> {
    path("assets")
        .and(tail())
        .and_then(async |tail: Tail| match ASSETS.get(tail.as_str()) {
            Some(data) => Ok(reply::with_header(
                *data,
                "Cache-Control",
                format!("max-age={}", 60 * 60 * 24 * 30),
            )),

            None => Err(reject::not_found()),
        })
        .boxed()
}
