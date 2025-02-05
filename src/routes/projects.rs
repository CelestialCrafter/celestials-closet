use askama_warp::Template;
use warp::reply::Reply;

struct Project<'a> {
    title: &'a str,
    description: &'a str,
    url: &'a str,
}

#[derive(Template)]
#[template(path = "projects.html")]
struct ProjectsTemplate<'a> {
    projects: Vec<Project<'a>>,
}

pub async fn page() -> impl Reply {
    let projects = vec![
    Project {
        title: "advent-of-code",
        description: "my advent of code attempts!",
        url: "https://github.com/CelestialCrafter/advent-of-code",
    },
    Project {
        title: "CelestialCrafter",
        description: "my readme!",
        url: "https://github.com/CelestialCrafter/CelestialCrafter",
    },
    Project {
        title: "celestials-closet",
        description: "my personal website!",
        url: "https://github.com/CelestialCrafter/celestials-closet",
    },
    Project {
        title: "chatscanner",
        description: "Scans for words said in chat in Minecraft",
        url: "https://github.com/CelestialCrafter/chatscanner",
    },
    Project {
        title: "crawler",
        description: "web crawler!",
        url: "https://github.com/CelestialCrafter/crawler",
    },
    Project {
        title: "cytrus",
        description: "A multipurpose bot to cover all your needs.",
        url: "https://github.com/CelestialCrafter/cytrus",
    },
    Project {
        title: "dotfiles",
        description: "my dotfiles!",
        url: "https://github.com/CelestialCrafter/dotfiles",
    },
    Project {
        title: "drawing-editor",
        description: "",
        url: "https://github.com/CelestialCrafter/drawing-editor",
    },
    Project {
        title: "fenlu",
        description: "Simple and Extensible all-purpose media organizer inspired by qimgv",
        url: "https://github.com/CelestialCrafter/fenlu",
    },
    Project {
        title: "games",
        description: "a collection of games playable over ssh!",
        url: "https://github.com/CelestialCrafter/games",
    },
    Project {
        title: "graphs",
        description: "collection of graph scripts",
        url: "https://github.com/CelestialCrafter/graphs",
    },
    Project {
        title: "groundx-code-samples",
        description: "",
        url: "https://github.com/CelestialCrafter/groundx-code-samples",
    },
    Project {
        title: "hsr-auto-auto-battle",
        description: "fully automatic battling for Honkai Star Rail",
        url: "https://github.com/CelestialCrafter/hsr-auto-auto-battle",
    },
    Project {
        title: "HypixelSBCalc",
        description: "Merchant & Minion Calculator for Hypixel Skyblock",
        url: "https://github.com/CelestialCrafter/HypixelSBCalc",
    },
    Project {
        title: "koharu",
        description: "System for ComputerCraft",
        url: "https://github.com/CelestialCrafter/koharu",
    },
    Project {
        title: "lang-guesser",
        description: "my (programming) language guessing game!",
        url: "https://github.com/CelestialCrafter/lang-guesser",
    },
    Project {
        title: "metrics",
        description: "my setup for metrics!",
        url: "https://github.com/CelestialCrafter/metrics",
    },
    Project {
        title: "ml-text-generation",
        description: "simple rnn to generate text",
        url: "https://github.com/CelestialCrafter/ml-text-generation",
    },
    Project {
        title: "nap.nvim",
        description: "Quickly move between next and previous NeoVim buffer, tab, file, quickfix, diagnostic, etc.",
        url: "https://github.com/CelestialCrafter/nap.nvim",
    },
    Project {
        title: "nixos-config",
        description: "my nixos configs!",
        url: "https://github.com/CelestialCrafter/nixos-config",
    },
    Project {
        title: "nixvim-config",
        description: "my nixvim config!",
        url: "https://github.com/CelestialCrafter/nixvim-config",
    },
    Project {
        title: "persistence.nvim",
        description: "💾  Simple session management for Neovim",
        url: "https://github.com/CelestialCrafter/persistence.nvim",
    },
    Project {
        title: "pixiv-sync",
        description: "",
        url: "https://github.com/CelestialCrafter/pixiv-sync",
    },
    Project {
        title: "platformergame",
        description: "randomly generated unity platformer game!",
        url: "https://github.com/CelestialCrafter/platformergame",
    },
    Project {
        title: "play-flac",
        description: "(very) simple script to play flac files thru alsa. made with gum, flac, and alsa-utils",
        url: "https://github.com/CelestialCrafter/play-flac",
    },
    Project {
        title: "qmk_firmware",
        description: "Open-source keyboard firmware for Atmel AVR and Arm USB families",
        url: "https://github.com/CelestialCrafter/qmk_firmware",
    },
    Project {
        title: "rp-client",
        description: "Uses Discord RPC to display what you're doing and a status to go along with it",
        url: "https://github.com/CelestialCrafter/rp-client",
    },
    Project {
        title: "search-engine",
        description: "",
        url: "https://github.com/CelestialCrafter/search-engine",
    },
    Project {
        title: "SkyStats",
        description: "hypixel skyblock inventory viewer",
        url: "https://github.com/CelestialCrafter/SkyStats",
    },
    Project {
        title: "someone",
        description: "A Simple discord bot that brings back @someone",
        url: "https://github.com/CelestialCrafter/someone",
    },
];
    ProjectsTemplate { projects }
}
