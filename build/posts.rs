use std::{env, fs, path::Path};

use eyre::{eyre, ErrReport, Result};
use pulldown_cmark::{CowStr, Event, LinkType, Options, Parser, Tag, TagEnd};
use tree_sitter_highlight::Highlighter;

use crate::{escape, hashmap, highlighting::inject_highlights};

const POSTS_DIR: &str = "posts";

fn inject_heading_links<'a>(
    parser: impl Iterator<Item = Event<'a>>,
) -> impl Iterator<Item = Event<'a>> {
    parser
        .scan(None, |state, event| {
            match event {
                Event::Text(ref text) => {
                    if let Some(Tag::Heading {
                        ref mut id, level, ..
                    }) = state
                    {
                        let new_id = text.to_lowercase().replace(" ", "-");
                        let link = Tag::Link {
                            link_type: LinkType::Inline,
                            dest_url: format!("#{}", new_id).into(),
                            title: CowStr::Borrowed(""),
                            id: CowStr::Borrowed(""),
                        };
                        *id = Some(new_id.into());
                        let level = level.clone();

                        return Some(vec![
                            Event::Start(state.take().unwrap()),
                            Event::Start(link),
                            event,
                            Event::End(TagEnd::Link),
                            Event::End(TagEnd::Heading(level)),
                        ]);
                    }
                }
                Event::Start(tag @ Tag::Heading { id: None, .. }) => {
                    *state = Some(tag);
                    return Some(vec![]);
                }
                Event::End(TagEnd::Heading(_)) => return Some(vec![]),
                _ => (),
            }

            Some(vec![event])
        })
        .flatten()
}

macro_rules! extract {
    ($frontmatter:expr, $conv:ident, $key:expr) => {
        $frontmatter
            .get($key)
            .ok_or(eyre!("post does not have {}", $key))?
            .$conv()
            .ok_or(eyre!("post {} is not string", $key))?
    };
}

pub fn process_posts() -> Result<()> {
    println!("cargo::rerun-if-changed={}", POSTS_DIR);

    let mut parser_opts = Options::empty();
    parser_opts.insert(Options::ENABLE_TABLES);
    parser_opts.insert(Options::ENABLE_STRIKETHROUGH);
    parser_opts.insert(Options::ENABLE_GFM);

    let mut highlighter = Highlighter::new();
    let entries = fs::read_dir(env::current_dir()?.join(POSTS_DIR))?
        .map(|entry| {
            let lines = fs::read_to_string(&entry?.path())?;
            let mut lines = lines.lines();
            let lines = lines.by_ref();

            // if statements are for losers
            let (frontmatter, content): (toml::Table, _) = (
                toml::from_str(
                    &lines
                        .scan(false, |state, line| match line == "---" {
                            true => match state {
                                true => None,
                                false => {
                                    *state = true;
                                    Some(None)
                                }
                            },
                            false => match state {
                                true => Some(Some(Ok(line))),
                                false => Some(Some(Err(eyre!("no metadata on post")))),
                            },
                        })
                        .filter_map(|v| v)
                        .collect::<Result<Vec<_>>>()?
                        .join("\n"),
                )?,
                lines.collect::<Vec<_>>().join("\n"),
            );

            let parser = {
                let original = Parser::new_ext(&content, parser_opts);
                let highlighted = inject_highlights(&mut highlighter, original);
                let linked = inject_heading_links(highlighted);

                linked
            };

            let mut html = String::new();
            pulldown_cmark::html::push_html(&mut html, parser);

            let id = escape(extract!(frontmatter, as_str, "id"));
            let date = {
                let date = extract!(frontmatter, as_datetime, "date")
                    .date
                    .ok_or(eyre!("post date is not a date"))?;

                escape(&format!("{}/{}/{}", date.year, date.month, date.day))
            };

            Ok(format!(
                "({id}, Post {{
                    id: {id},
                    date: {date},
                    title: {},
                    summary: {},
                    html: {},
                }})",
                escape(extract!(frontmatter, as_str, "title")),
                escape(extract!(frontmatter, as_str, "summary")),
                escape(&html),
            ))
        })
        .map(|v| v.inspect_err(|err: &ErrReport| eprintln!("could not process post: {err}")))
        .collect::<Result<Vec<_>>>()
        .map_err(|_| eyre!("could not process assets"))?;

    fs::write(
        Path::new(&env::var("OUT_DIR")?).join("posts.rs"),
        format!(
            "struct Post<'a> {{
                title: &'a str,
                id: &'a str,
                date: &'a str,
                summary: &'a str,
                html: &'a str,
            }}

            {}",
            hashmap("POSTS", "&str, Post", entries.into_iter())
        ),
    )?;

    Ok(())
}
