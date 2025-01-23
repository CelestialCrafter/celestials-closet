use warp::{
    http::StatusCode,
    reject::Rejection,
    reply::{self, Reply},
};

pub async fn handle(rejection: Rejection) -> Result<impl Reply, Rejection> {
    if rejection.is_not_found() {
        Ok(reply::with_status(
            "route not found!!",
            StatusCode::NOT_FOUND,
        ))
    } else {
        Err(rejection)
    }
}
