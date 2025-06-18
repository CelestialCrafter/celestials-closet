use askama::Template;
use eyre::Result;
use warp::{
    filters::BoxedFilter,
    path, reject,
    reply::{html, Reply},
    Filter,
};

include!(concat!(env!("OUT_DIR"), "/posts.rs"));

pub fn route() -> BoxedFilter<(impl Reply,)> {
    let listing = path::end().then(listing);
    let post = path::param().and_then(post);

    path("posts").and(listing.or(post)).boxed()
}

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

async fn listing() -> impl Reply {
    html(
        PostsTemplate {
            posts: POSTS.values().collect(),
        }
        .render()
        .expect("template should render"),
    )
}

async fn post(name: String) -> Result<impl Reply, reject::Rejection> {
    match POSTS.get(name.as_str()) {
        Some(post) => Ok(html(
            PostTemplate { post }
                .render()
                .expect("template should render"),
        )),
        None => Err(reject::not_found()),
    }
}
