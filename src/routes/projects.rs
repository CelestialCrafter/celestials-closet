use askama_warp::Template;
use warp::reply::Reply;

struct Project<'a> {
    title: &'a str,
    description: &'a str,
    url: &'a str,
    images: Option<&'a [&'a str]>,
}

#[derive(Template)]
#[template(path = "projects.html")]
struct ProjectsTemplate<'a> {
    projects: Vec<Project<'a>>,
}

pub async fn page() -> impl Reply {
    let projects = vec![
        Project {
            title: "celestials-closet",
            description: "my personal website!",
            url: "https://github.com/CelestialCrafter/celestials-closet",
            images: None,
        },
        Project {
            title: "dotfiles",
            description: "my dotfiles!",
            url: "https://github.com/CelestialCrafter/dotfiles",
            images: Some(&[
                "https://github.com/user-attachments/assets/879768e4-5a9f-4207-9e7e-7f6eaa9d0f18",
                "https://github.com/CelestialCrafter/dotfiles/assets/44733683/b67aab0c-29d5-4f4e-a5c4-7ee4eaf08fca"
            ]),
        },
        Project {
            title: "games",
            description: "a collection of games playable over ssh!",
            url: "https://github.com/CelestialCrafter/games",
            images: None,
        },
        Project {
            title: "crawler",
            description: "web crawler!",
            url: "https://github.com/CelestialCrafter/crawler",
            images: None,
        },
        Project {
            title: "fenlu",
            description: "simple and extensible all-purpose media organizer inspired by qimgv",
            url: "https://github.com/CelestialCrafter/fenlu",
            images: None,
        },
        Project {
            title: "koharu",
            description: "System for ComputerCraft",
            url: "https://github.com/CelestialCrafter/koharu",
            images: None,
        },
        Project {
            title: "rp-client",
            description:
                "Uses Discord RPC to display what you're doing and a status to go along with it",
            url: "https://github.com/CelestialCrafter/rp-client",
            images: None,
        },
    ];
    ProjectsTemplate { projects }
}
