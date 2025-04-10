use askama_warp::Template;
use warp::{
    filters::{path::path, BoxedFilter},
    reply::Reply,
    Filter,
};

pub fn route() -> BoxedFilter<(impl Reply,)> {
    path("personal").then(page).boxed()
}

struct Interest<'a> {
    title: &'a str,
    image: &'a str,
    url: &'a str,
}

struct Song<'a> {
    title: &'a str,
    artist: &'a str,
    url: &'a str,
}

#[derive(Template)]
#[template(path = "personal.html")]
struct PersonalTemplate<'a> {
    interests: Vec<Interest<'a>>,
    songs: Vec<Song<'a>>,
}

async fn page() -> impl Reply {
    PersonalTemplate {
        interests: vec![
            Interest {
                title: "Yorushika",
                image: "/assets/yorushika.webp",
                url: "https://yorushika.com",
            },
            Interest {
                title: "Frieren",
                image: "/assets/frieren.webp",
                url: "https://frieren-anime.jp",
            },
            Interest {
                title: "The Apothecary Diaries",
                image: "/assets/apothecary-diaries.webp",
                url: "https://kusuriyanohitorigoto.jp",
            },
            Interest {
                title: "NieR: Automata",
                image: "/assets/nier.webp",
                url: "https://nierautomata.square-enix-games.com",
            },
            Interest {
                title: "NEEDY STREAMER OVERLOAD",
                image: "/assets/needy.webp",
                url: "https://whysoserious.jp/needy/en",
            },
        ],
        songs: vec![
            Song {
                title: "言って。",
                artist: "Yorushika",
                url: "https://www.youtube.com/watch?v=F64yFFnZfkI",
            },
            Song {
                title: "Oceans & Engines",
                artist: "Niki",
                url: "https://www.youtube.com/watch?v=DropwjmHtoo",
            },
            Song {
                title: "八月、某、月明かり",
                artist: "Yorushika",
                url: "https://open.spotify.com/track/4AK6TjpIybCTe1QphPXFn9",
            },
            Song {
                title: "Anaheim",
                artist: "Niki",
                url: "https://open.spotify.com/track/5GoY2ioRnfQayqsNx4HaXh",
            },
            Song {
                title: "へび",
                artist: "Yorushika",
                url: "https://open.spotify.com/track/7pk2Mx1LnlaEpxfzNhgRuz",
            },
            Song {
                title: "unravel",
                artist: "Ado",
                url: "https://open.spotify.com/track/3uyiyeKrVPjbMdFGe9sGas",
            },
            Song {
                title: "urs",
                artist: "Niki",
                url: "https://open.spotify.com/track/4EMTe461jubpxPqFfVA0Rp",
            },
        ],
    }
}
