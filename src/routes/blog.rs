use askama_warp::Template;
use warp::{
    reject::{self, Rejection},
    reply::Reply,
};

include!(concat!(env!("OUT_DIR"), "/blogs.rs"));

#[derive(Template)]
#[template(path = "blog.html")]
struct BlogTemplate<'a> {
    data: &'a str,
    css: &'a str,
}

pub async fn page(name: String) -> Result<impl Reply, Rejection> {
    match BLOGS.get(name.as_str()) {
        Some(content) => Ok(BlogTemplate {
            data: content,
            css: grass::include!("styles/blog.scss"),
        }),
        None => Err(reject::not_found()),
    }
}
