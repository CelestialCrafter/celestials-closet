use warp::{filters, reply::Reply, Filter};

use crate::assets;

pub mod database;
pub mod index;
pub mod personal;
pub mod posts;
pub mod projects;
pub mod rejections;

pub fn routes() -> filters::BoxedFilter<(impl Reply,)> {
    assets::route()
        .or(index::route())
        .or(projects::route())
        .or(posts::route())
        .or(personal::route())
        .recover(rejections::handle)
        .with(filters::compression::gzip())
        .boxed()
}
