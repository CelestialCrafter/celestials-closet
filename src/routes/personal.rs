use askama::Template;
use warp::{
    filters::{path::path, BoxedFilter},
    reply::{html, Reply},
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
    html(PersonalTemplate {
        games: vec![
            Resource {
                title: "NieR: Automata",
                image: "/assets/nier-automata.webp",
                url: "https://nierautomata.square-enix-games.com",
            },
            Resource {
                title: "NEEDY STREAMER OVERLOAD",
                image: "/assets/needy-streamer-overload.webp",
                url: "https://whysoserious.jp/needy/en",
            },
            Resource {
                title: "Persona 5 Royal",
                image: "/assets/persona-5-royal.webp",
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
                image: "/assets/bocchi-the-rock.webp",
                url: "https://bocchi.rocks/",
            },
            Resource {
                title: "Girls Band Cry",
                image: "/assets/girls-band-cry.webp",
                url: "https://girls-band-cry.com/",
            },
            Resource {
                title: "Lycoris Recoil",
                image: "/assets/lycoris-recoil.webp",
                url: "https://lycorisrecoil.com/",
            },
            Resource {
                title: "Witch Hat Atelier",
                image: "/assets/witch-hat-atlier.webp",
                url: "https://tongari-anime.com/en/",
            },
            Resource {
                title: "Oshi no Ko",
                image: "/assets/oshi-no-ko.webp",
                url: "https://ichigoproduction.com/",
            },
        ],
    }.render().expect("template should render"))
}
