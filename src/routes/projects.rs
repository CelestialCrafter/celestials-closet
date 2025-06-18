use std::fmt;

use askama::Template;
use warp::{
    filters::{path::path, BoxedFilter},
    reply::{html, Reply},
    Filter,
};

pub fn route() -> BoxedFilter<(impl Reply,)> {
    path("projects").then(page).boxed()
}

#[derive(Default)]
struct Language<'a>(Box<[&'a str]>);

impl Language<'_> {
    fn known(&self) -> bool {
        self.0.len() > 0
    }
}

impl fmt::Display for Language<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.len() {
            0 => write!(f, "Unknown"),
            1.. => write!(f, "{}", self.0.join("/")),
        }
    }
}

impl<'a, const N: usize> From<[&'a str; N]> for Language<'a> {
    fn from(value: [&'a str; N]) -> Self {
        Self(value.into())
    }
}

impl<'a> From<&'a str> for Language<'a> {
    fn from(value: &'a str) -> Self {
        Self([value].into())
    }
}

struct Project<'a> {
    title: &'a str,
    description: &'a str,
    url: &'a str,
    language: Language<'a>,
    images: Option<&'a [&'a str]>,
}

#[derive(Template)]
#[template(path = "projects.html")]
struct ProjectsTemplate<'a> {
    projects: Vec<Project<'a>>,
}

async fn page() -> impl Reply {
    html(ProjectsTemplate {
        projects: vec![
            Project {
                title: "celestials-closet",
                description: "my personal website!",
                url: "https://github.com/CelestialCrafter/celestials-closet",
                language: "Rust".into(),
                images: None,
            },
            Project {
                title: "dotfiles",
                description: "my dotfiles!",
                url: "https://github.com/CelestialCrafter/dotfiles",
                language: Language::default(),
                images: Some(&["/assets/dotfiles-1.webp", "/assets/dotfiles-2.webp"]),
            },
            Project {
                title: "games",
                description: "a collection of games playable over ssh!",
                url: "https://github.com/CelestialCrafter/games",
                language: "Go".into(),
                images: None,
            },
            Project {
                title: "crawler",
                description: "web crawler!",
                url: "https://github.com/CelestialCrafter/crawler",
                language: "Go".into(),
                images: None,
            },
            Project {
                title: "fenlu",
                description: "simple and extensible all-purpose media organizer inspired by qimgv",
                url: "https://github.com/CelestialCrafter/fenlu",
                language: ["Go", "Rust", "Python"].into(),
                images: None,
            },
            Project {
                title: "koharu",
                description: "System for ComputerCraft",
                url: "https://github.com/CelestialCrafter/koharu",
                language: "Lua".into(),
                images: None,
            },
            Project {
                title: "rp-client",
                description:
                    "Uses Discord RPC to display what you're doing and a status to go along with it",
                url: "https://github.com/CelestialCrafter/rp-client",
                language: "Javascript".into(),
                images: None,
            },
        ],
    }.render().expect("template should render"))
}
