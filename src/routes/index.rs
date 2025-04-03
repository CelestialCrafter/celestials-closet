use askama_warp::Template;
use warp::reply::Reply;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    views: u64,
}

pub async fn page(views: u64) -> impl Reply {
    IndexTemplate { views }
}
