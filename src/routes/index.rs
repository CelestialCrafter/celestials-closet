use askama::Template;
use warp::{
    filters,
    reply::{html, Reply},
    Filter,
};

pub fn route() -> filters::BoxedFilter<(impl Reply,)> {
    warp::path::end().then(page).boxed()
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

async fn page() -> impl Reply {
    html(IndexTemplate.render().expect("template should render"))
}
