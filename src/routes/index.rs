use askama_warp::Template;
use grass::include;
use warp::reply::Reply;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    css: &'a str,
}

pub async fn page() -> impl Reply {
    IndexTemplate {
        css: include!("styles/index.scss"),
    }
}
