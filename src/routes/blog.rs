use std::process::Command;

use askama_warp::Template;
use eyre::{eyre, Result};
use warp::{
    reject::{self, Rejection},
    reply::Reply,
};

use crate::errors::Failed;

include!(concat!(env!("OUT_DIR"), "/blogs.rs"));

struct Revision {
    jj: String,
    git: String,
    timestamp: u64,
}

#[derive(Template)]
#[template(path = "blog.html")]
struct BlogTemplate<'a> {
    data: &'a str,
    revision: Revision,
}

// gross but adding jj_lib makes the build time so long
fn generate_revision(path: &str) -> Result<Revision> {
    let common_args = [
        "--ignore-working-copy",
        "--no-pager",
        "--quiet",
        "--color",
        "never",
    ];

    // get revision
    let annotate = Command::new("jj")
        .args(["file", "annotate", path])
        .args(common_args)
        .output()?;
    if !annotate.status.success() {
        Err(eyre!("jj errored: {}", String::from_utf8(annotate.stderr)?))?;
    }

    let stdout = String::from_utf8(annotate.stdout)?;
    let (revision, _) = stdout
        .split_once(' ')
        .ok_or(eyre!("no space delimiter found"))?;

    // get commit info
    let show = Command::new("jj")
        .args([
            "show",
            revision,
            "--template",
            r#"commit_id.short() ++ " " ++ change_id.short() ++ " " ++ author.timestamp().utc().format("%s") ++ " ""#,
        ])
        .args(common_args)
        .output()?;
    if !show.status.success() {
        return Err(eyre!("jj errored: {}", String::from_utf8(show.stderr)?));
    }

    let stdout = String::from_utf8(show.stdout)?;
    let split: Vec<&str> = stdout.splitn(4, ' ').collect();
    if split.len() != 4 {
        return Err(eyre!("split did not have 4 elements"));
    }

    Ok(Revision {
        git: split[0].to_string(),
        jj: split[1].to_string(),
        timestamp: split[2].parse()?,
    })
}

pub async fn page(name: String) -> Result<impl Reply, Rejection> {
    match BLOGS.get(name.as_str()) {
        Some((path, content)) => Ok(BlogTemplate {
            data: content,
            revision: generate_revision(path).map_err(|err| reject::custom(Failed(err)))?,
        }),
        None => Err(reject::not_found()),
    }
}
