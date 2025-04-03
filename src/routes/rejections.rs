use warp::{
    http::StatusCode,
    reject::Rejection,
    reply::{with_status, Reply},
};

pub async fn handle(rejection: Rejection) -> Result<impl Reply, Rejection> {
    if rejection.is_not_found() {
        Ok(with_status(
            "route not found!!".to_string(),
            StatusCode::NOT_FOUND,
        ).into_response())
    } else {
        Err(rejection)
    }
}
