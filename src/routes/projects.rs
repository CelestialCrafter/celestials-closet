use std::sync::Arc;

use askama_warp::Template;
use eyre::Result;
use reqwest::Client;
use serde::Deserialize;
use warp::reply::Reply;

const GITHUB_USERNAME: &str = "CelestialCrafter";

#[derive(Debug, Deserialize, Clone)]
pub struct Repository {
    name: String,
    #[serde(rename(deserialize = "html_url"))]
    url: String,
    language: Option<String>,
    #[serde(rename(deserialize = "stargazers_count"))]
    stars: usize,
}

// @TODO pagination?
pub async fn repositories() -> Result<Vec<Repository>> {
    let client = Client::new();
    let json = client
        .get(format!(
            "https://api.github.com/users/{}/repos?per_page=100&sort=updated",
            GITHUB_USERNAME
        ))
        .header("User-Agent", "CelestialsCloset")
        .header("Accept", "application/vnd.github+json")
        .send()
        .await?
        .json::<Vec<Repository>>()
        .await?;

    Ok(json)
}

#[derive(Template)]
#[template(path = "projects.html")]
struct ProjectsTemplate<'a> {
    css: &'a str,
    repos: Arc<Vec<Repository>>,
}

pub async fn page(repos: Arc<Vec<Repository>>) -> impl Reply {
    ProjectsTemplate {
        css: grass::include!("styles/projects.scss"),
        repos,
    }
}
