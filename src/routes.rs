use std::net::SocketAddr;

use warp::{filters, reply::Reply, Filter};

use crate::{assets, database::Database};

pub mod database;
pub mod index;
pub mod posts;
pub mod projects;
pub mod rejections;

pub fn routes(db: &'static Database) -> filters::BoxedFilter<(impl Reply,)> {
    let count_view = warp::any()
        .and(filters::addr::remote())
        .map(move |addr: Option<SocketAddr>| {
            if let Some(addr) = addr {
                db.store(addr.ip(), None).unwrap();
            }
        })
        .untuple_one();

    assets::route()
        .or(count_view
            .and(index::route(db))
            .or(projects::route())
            .or(posts::route()))
        .recover(rejections::handle)
        .with(filters::compression::gzip())
        .boxed()
}
