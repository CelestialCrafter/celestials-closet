use askama_warp::Template;
use warp::{
    filters::{path::path, BoxedFilter},
    reply::Reply,
    Filter,
};

pub fn route() -> BoxedFilter<(impl Reply,)> {
    path("personal").then(page).boxed()
}

struct Resource<'a> {
    title: &'a str,
    image: &'a str,
    url: &'a str,
}

#[derive(Template)]
#[template(path = "personal.html")]
struct PersonalTemplate<'a> {
    games: Vec<Resource<'a>>,
    music: Vec<Resource<'a>>,
    anime_manga: Vec<Resource<'a>>,
}

async fn page() -> impl Reply {
    PersonalTemplate {
        games: vec![
            Resource {
                title: "NieR: Automata",
                image: "/assets/nier.webp",
                url: "https://nierautomata.square-enix-games.com",
            },
            Resource {
                title: "NEEDY STREAMER OVERLOAD",
                image: "/assets/needy.webp",
                url: "https://whysoserious.jp/needy/en",
            },
            Resource {
                title: "Persona 5 Royal",
                image: "/assets/p5r.webp",
                url: "https://persona.atlus.com/p5r",
            }
        ],
        music: vec![
            Resource {
                title: "Yorushika",
                image: "/assets/yorushika.webp",
                url: "https://yorushika.com",
            },
            Resource {
                title: "NIKI",
                image: "/assets/niki.webp",
                url: "https://nikizefanya.com",
            },
            Resource {
                title: "Ado",
                image: "/assets/ado.webp",
                url: "http://universal-music.co.jp/ado",
            },
            Resource {
                title: "ZUTOMAYO",
                image: "/assets/zutomayo.webp",
                url: "https://zutomayo.net",
            },
            Resource {
                title: "beabadoobee",
                image: "/assets/beabadoobee.webp",
                url: "https://www.beabadoobee.com",
            },
            Resource {
                title: "INABAKUMORI",
                image: "/assets/inabakumori.webp",
                url: "https://www.youtube.com/@Inabakumori",
            }
        ],
        anime_manga: vec![
            Resource {
                title: "Frieren",
                image: "/assets/frieren.webp",
                url: "https://frieren-anime.jp/",
            },
            Resource {
                title: "Bocchi the Rock!",
                image: "/assets/bocchi.webp",
                url: "https://bocchi.rocks/",
            },
            Resource {
                title: "Girls Band Cry",
                image: "/assets/gbc.webp",
                url: "https://girls-band-cry.com/",
            },
            Resource {
                title: "Lycoris Recoil",
                image: "/assets/lycoris.webp",
                url: "https://lycorisrecoil.com/",
            },
            Resource {
                title: "Witch Hat Atelier",
                image: "/assets/atlier.webp",
                url: "https://tongari-anime.com/en/",
            },
            Resource {
                title: "Oshi no Ko",
                image: "/assets/onk.webp",
                url: "https://ichigoproduction.com/",
            },
            Resource {
                title: "Bloom into You",
                image: "/assets/bloom.webp",
                url: "https://yagakimi.fandom.com/wiki/Bloom_Into_You",
            },
        ],
    }
}
