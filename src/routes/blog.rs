use askama_warp::Template;
use eyre::Result;
use warp::{
    reject::{self, Rejection},
    reply::Reply,
};

include!(concat!(env!("OUT_DIR"), "/posts.rs"));

#[derive(Template)]
#[template(path = "post.html")]
struct PostTemplate<'a> {
    post: &'a Post<'a>,
}

#[derive(Template)]
#[template(path = "posts.html")]
struct PostsTemplate<'a> {
    posts: Vec<&'a Post<'a>>,
}

pub async fn page() -> impl Reply {
    PostsTemplate {
        posts: POSTS.values().collect(),
    }
}

pub async fn post_page(name: String) -> Result<impl Reply, Rejection> {
    match POSTS.get(name.as_str()) {
        Some(post) => Ok(PostTemplate { post }),
        None => Err(reject::not_found()),
    }
}
