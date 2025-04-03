use std::sync::Arc;

use warp::{filters, path, reply, Filter};

use crate::database::Database;

pub mod database;
pub mod index;
pub mod posts;
pub mod projects;
pub mod rejections;

pub fn routes(db: Arc<Database>) -> filters::BoxedFilter<(impl reply::Reply,)> {
    let index = path::end().map(move || db.increment()).then(index::page);
    let projects = path("projects").then(projects::page);

    let posts = {
        let listing = path::end().then(posts::listing);
        let post = path::param().and_then(posts::post);

        path("posts").and(listing.or(post))
    };

    let assets = path("assets").and(filters::fs::dir("assets")).map(|reply| {
        reply::with_header(
            reply,
            "Cache-Control",
            format!("max-age={}", 60 * 60 * 24 * 7),
        )
    });

    let root = assets.or(index).or(projects).or(posts);
    root.recover(rejections::handle)
        .with(filters::compression::brotli())
        .boxed()
}
