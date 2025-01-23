use askama_warp::Template;
use warp::reply::Reply;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

pub async fn page() -> impl Reply {
    IndexTemplate {}
}
