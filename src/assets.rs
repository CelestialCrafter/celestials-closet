use warp::{
    filters::path::Tail,
    reject::{self, Rejection},
    reply::Reply,
};

include!(concat!(env!("OUT_DIR"), "/assets.rs"));

pub async fn page(tail: Tail) -> Result<impl Reply, Rejection> {
    match ASSETS.get(tail.as_str()) {
        Some(data) => Ok(data.clone()),
        None => Err(reject::not_found()),
    }
}
