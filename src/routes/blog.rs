use askama_warp::Template;
use eyre::Result;
use warp::{
    reject::{self, Rejection},
    reply::Reply,
};

include!(concat!(env!("OUT_DIR"), "/blogs.rs"));

#[derive(Template)]
#[template(path = "blog_post.html")]
struct BlogPostTemplate<'a> {
    blog: &'a Blog<'a>,
}

#[derive(Template)]
#[template(path = "blog.html")]
struct BlogTemplate<'a> {
    posts: Vec<&'a Blog<'a>>,
}

pub async fn page() -> impl Reply {
    BlogTemplate {
        posts: BLOGS.values().collect(),
    }
}

pub async fn post_page(name: String) -> Result<impl Reply, Rejection> {
    match BLOGS.get(name.as_str()) {
        Some(blog) => Ok(BlogPostTemplate { blog }),
        None => Err(reject::not_found()),
    }
}
