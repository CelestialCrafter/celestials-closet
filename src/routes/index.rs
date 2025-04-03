use std::{collections::HashMap, net::SocketAddr};

use askama_warp::Template;
use warp::{
    filters,
    http::StatusCode,
    reply::{with_status, Reply},
    Filter,
};

use crate::database::{Database, Message};

pub fn route(db: &'static Database) -> filters::BoxedFilter<(impl Reply,)> {
    warp::path::end()
        .and(
            filters::method::post()
                .map(move || db)
                .and(filters::body::form())
                .and(filters::addr::remote())
                .then(post)
        .or(warp::any().map(move || db).then(page)),
        )
        .boxed()
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    views: usize,
    messages: Box<[Message]>,
}

async fn post(
    db: &'static Database,
    mut form: HashMap<String, String>,
    addr: Option<SocketAddr>,
) -> impl Reply {
    let addr = match addr {
        Some(addr) => addr,
        None => {
            return Err(with_status(
                "server transport does not use socket addresses",
                StatusCode::INTERNAL_SERVER_ERROR,
            )
            .into_response())
        }
    };

    let name = form
        .remove("name")
        .ok_or(with_status("missing name field", StatusCode::BAD_REQUEST).into_response())?;
    let content = form
        .remove("content")
        .ok_or(with_status("missing msg field", StatusCode::BAD_REQUEST).into_response())?;

    db.store(addr.ip(), Some(Message { name, content }))
        .map_err(|err| with_status(err.to_string(), StatusCode::BAD_REQUEST).into_response())?;

    Ok(page(db).await)
}

async fn page(db: &Database) -> impl Reply + use<'_> {
    let db = db.read().unwrap();
    IndexTemplate {
        views: db.len(),
        messages: db.values().cloned().filter_map(|v| v).collect(),
    }
}
