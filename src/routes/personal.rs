use askama::Template;
use warp::{
    Filter,
    filters::{BoxedFilter, path::path},
    reply::{Reply, html},
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
    music: Vec<Resource<'a>>,
    anime_manga: Vec<Resource<'a>>,
}

async fn page() -> impl Reply {
    html(
        PersonalTemplate {
            music: vec![
                Resource {
                    title: "Yorushika",
                    image: "/assets/yorushika.webp",
                    url: "https://yorushika.com/",
                },
                Resource {
                    title: "INABAKUMORI",
                    image: "/assets/inabakumori.webp",
                    url: "https://www.youtube.com/@Inabakumori",
                },
                Resource {
                    title: "ZUTOMAYO",
                    image: "/assets/zutomayo.webp",
                    url: "https://zutomayo.net/",
                },
                Resource {
                    title: "kessoku band",
                    image: "/assets/kessoku-band.webp",
                    url: "https://bocchi.rocks/kessokuband/discography/",
                },
                Resource {
                    title: "Natori",
                    image: "/assets/natori.webp",
                    url: "https://natori-official.com/",
                },
                Resource {
                    title: "DECO*27",
                    image: "/assets/deco-27.webp",
                    url: "https://otoiro.co.jp/creator/deco27/",
                },
                Resource {
                    title: "NIKI",
                    image: "/assets/niki.webp",
                    url: "https://nikizefanya.com/",
                },
                Resource {
                    title: "Iyowa",
                    image: "/assets/iyowa.webp",
                    url: "https://www.youtube.com/@igusuri_please",
                },
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
                    url: "https://tongari-anime.com/en",
                },
                Resource {
                    title: "Oshi no Ko",
                    image: "/assets/oshi-no-ko.webp",
                    url: "https://ichigoproduction.com/",
                },
                Resource {
                    title: "Takopi's Original Sin",
                    image: "/assets/takopi.webp",
                    url: "https://www.tbs.co.jp/anime/takopi_project",
                },
                Resource {
                    title: "Makeine",
                    image: "/assets/makeine.webp",
                    url: "https://makeine-anime.com/",
                },
                Resource {
                    title: "The Summer Hikaru Died",
                    image: "/assets/hikaru.webp",
                    url: "https://hikanatsu-anime.com/",
                },
                Resource {
                    title: "Cosmic Princess Kaguya",
                    image: "/assets/cosmic-princess-kaguya.webp",
                    url: "https://cho-kaguyahime.com/",
                },
            ],
        }
        .render()
        .expect("template should render"),
    )
}
