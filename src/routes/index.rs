use askama_warp::Template;
use warp::{
    filters,
    reply::Reply,
    Filter,
};

pub fn route() -> filters::BoxedFilter<(impl Reply,)> {
    warp::path::end()
        .then(page)
        .boxed()
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

async fn page() -> impl Reply {
    IndexTemplate
}
