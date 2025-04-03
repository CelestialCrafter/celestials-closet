use std::{fmt::Debug, ops::Deref};

use eyre::ErrReport;
use warp::{
    http::StatusCode,
    reject::{Reject, Rejection},
    reply,
};

pub struct AppError(pub ErrReport);

impl Debug for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl Deref for AppError {
    type Target = ErrReport;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Reject for AppError {}

pub async fn handle(rejection: Rejection) -> Result<impl reply::Reply, Rejection> {
    if rejection.is_not_found() {
        Ok(reply::with_status(
            "route not found!!",
            StatusCode::NOT_FOUND,
        ))
    } else {
        Err(rejection)
    }
}
