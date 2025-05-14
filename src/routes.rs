use std::net::SocketAddr;

use warp::{filters, reply::Reply, Filter};

use crate::{assets, DATABASE};

pub mod database;
pub mod index;
pub mod posts;
pub mod projects;
pub mod personal;
pub mod rejections;

pub fn routes() -> filters::BoxedFilter<(impl Reply,)> {
    let count_view = warp::any()
        .and(filters::addr::remote())
        .map(move |addr: Option<SocketAddr>| {
            if let (Some(addr), Some(db)) = (addr, DATABASE.get()) {
                db.store(addr.ip());
            }
        })
        .untuple_one();

    assets::route()
        .or(count_view
            .and(index::route())
            .or(projects::route())
            .or(posts::route()))
            .or(personal::route())
        .recover(rejections::handle)
        .with(filters::compression::gzip())
        .boxed()
}
