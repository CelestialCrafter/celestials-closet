use std::{env, fs, path::Path, process::Command};

use eyre::{eyre, Result};
use pulldown_cmark::{Options, Parser};
use tree_sitter_highlight::Highlighter;

use crate::{escape, highlighting::inject_highlights, map};

const POSTS_DIR: &str = "posts";

// @HACK gross but adding jj_lib makes the build time so long
fn get_revision(path: &str) -> Result<(String, u64)> {
    let common_args = [
        "--ignore-working-copy",
        "--no-pager",
        "--quiet",
        "--color",
        "never",
    ];

    // get revision
    let change = {
        let annotate = Command::new("jj")
            .args(["file", "annotate", path])
            .args(common_args)
            .output()?;
        if !annotate.status.success() {
            Err(eyre!("jj errored: {}", String::from_utf8(annotate.stderr)?))?;
        }

        let stdout = String::from_utf8(annotate.stdout)?;
        let (revision, _) = stdout
            .split_once(' ')
            .ok_or(eyre!("no space delimiter found"))?;

        revision.to_string()
    };

    let timestamp = {
        let show = Command::new("jj")
            .args([
                "show",
                &change,
                "--template",
                r#"author.timestamp().utc().format("%s") ++ " ""#,
            ])
            .args(common_args)
            .output()?;
        if !show.status.success() {
            return Err(eyre!("jj errored: {}", String::from_utf8(show.stderr)?));
        }

        let stdout = String::from_utf8(show.stdout)?;
        let (timestamp, _) = stdout
            .split_once(' ')
            .ok_or(eyre!("could not find space delimiter"))?;

        timestamp.parse()?
    };

    Ok((change, timestamp))
}

pub fn process_posts() -> Result<()> {
    println!("cargo::rerun-if-changed={}", POSTS_DIR);

    let mut parser_opts = Options::empty();
    parser_opts.insert(Options::ENABLE_TABLES);
    parser_opts.insert(Options::ENABLE_STRIKETHROUGH);
    parser_opts.insert(Options::ENABLE_GFM);

    let mut highlighter = Highlighter::new();
    let entries = fs::read_dir(env::current_dir()?.join(POSTS_DIR))?
        .into_iter()
        .map(|entry| {
            let path = entry?.path();
            let name = path
                .file_stem()
                .ok_or(eyre!("no stem on path"))?
                .to_str()
                .ok_or(eyre!("file stem is not utf-8"))?;
            let path = path.to_str().ok_or(eyre!("file path is not utf-8"))?;
            let input = fs::read_to_string(path)?;

            let mut html = String::new();
            pulldown_cmark::html::push_html(
                &mut html,
                inject_highlights(&mut highlighter, Parser::new_ext(&input, parser_opts)),
            );

            let id = escape(name);
            let rev = get_revision(path)?;

            let mut lines = input.lines();
            let (_, title) = lines
                .next()
                .unwrap_or_default()
                .split_once("# ")
                .unwrap_or_default();
            let summary = lines.find(|line| !line.is_empty()).unwrap_or_default();

            Ok(format!(
                "{}, Post {{
                    id: {},
                    title: {},
                    summary: {},
                    html: {},
                    revision: Revision {{
                        change: {},
                        timestamp: {}
                    }}
                }}",
                id,
                id,
                escape(title),
                escape(&summary),
                escape(&html),
                escape(&rev.0),
                rev.1
            ))
        })
        .collect::<Result<Vec<String>>>()?;

    fs::write(
        Path::new(&env::var("OUT_DIR")?).join("posts.rs"),
        format!(
            "{}{}",
            "struct Post<'a> {
                title: &'a str,
                id: &'a str,
                summary: &'a str,
                html: &'a str,
                revision: Revision<'a>,
            }

            struct Revision<'a> {
                change: &'a str,
                timestamp: u64,
            }",
            map("POSTS", "&str, Post", entries.into_iter())
        ),
    )?;

    Ok(())
}
