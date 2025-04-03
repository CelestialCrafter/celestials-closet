use askama_warp::Template;
use eyre::Result;
use warp::{
    reject,
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

pub async fn listing() -> impl Reply {
    PostsTemplate {
        posts: POSTS.values().collect(),
    }
}

pub async fn post(name: String) -> Result<impl Reply, reject::Rejection> {
    match POSTS.get(name.as_str()) {
        Some(post) => Ok(PostTemplate { post }),
        None => Err(reject::not_found()),
    }
}
