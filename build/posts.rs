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

pub fn process_posts() -> Result<()> {
    println!("cargo::rerun-if-changed={}", POSTS_DIR);

    let mut parser_opts = Options::empty();
    parser_opts.insert(Options::ENABLE_TABLES);
    parser_opts.insert(Options::ENABLE_STRIKETHROUGH);
    parser_opts.insert(Options::ENABLE_GFM);

    let mut highlighter = Highlighter::new();
    let entries = fs::read_dir(env::current_dir()?.join(POSTS_DIR))?
        .map(|entry| {
            // paths
            let path = entry?.path();

            let id = path
                .file_stem()
                .ok_or(eyre!("path does not have file name"))?
                .to_str()
                .ok_or(eyre!("could not convert stem to utf8"))?;

            let data = fs::read_to_string(&path)?;

            let parser = Parser::new_ext(&data, parser_opts);
            let parser = inject_highlights(&mut highlighter, parser);
            let parser = inject_heading_links(parser);

            let (mut title, mut summary) = (None, None);
            let parser = parser.map(|event| {
                if let Event::Text(ref text) = event {
                    if title.is_none() {
                        title = Some(text.clone());
                    } else if summary.is_none() {
                        summary = Some(text.clone());
                    };
                }

                event
            });

            let mut html = String::new();
            pulldown_cmark::html::push_html(&mut html, parser);

            Ok(format!(
                "({}, Post {{
                    id: {},
                    title: {},
                    summary: {},
                    html: {},
                }})",
                escape(id),
                escape(id),
                escape(&title.ok_or(eyre!("post did not have title"))?),
                escape(&summary.ok_or(eyre!("post did not have summary"))?),
                escape(&html),
            ))
        })
        .map(|v| {
            v.inspect_err(|err: &ErrReport| eprintln!("could not process post: {err}"))
        })
        .collect::<Result<Vec<_>>>()
        .map_err(|_| eyre!("could not process assets"))?;

    fs::write(
        Path::new(&env::var("OUT_DIR")?).join("posts.rs"),
        format!(
            "{}{}",
            "struct Post<'a> {
                title: &'a str,
                id: &'a str,
                summary: &'a str,
                html: &'a str,
            }",
            hashmap("POSTS", "&str, Post", entries.into_iter())
        ),
    )?;

    Ok(())
}
