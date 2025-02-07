use std::{
    env,
    fs::{self},
    path::{Path, PathBuf},
    process::Command,
};

use eyre::{eyre, Result};
use proc_macro2::Literal;
use pulldown_cmark::{Options, Parser};
use walkdir::WalkDir;

const POSTS_DIR: &str = "posts";
const ASSETS_DIR: &str = "assets";

fn escape(input: &str) -> String {
    Literal::string(input).to_string()
}

fn map(name: &str, map_sig: &str, entries: impl Iterator<Item = String>) -> String {
    format!(
        "use std::{{sync::LazyLock, collections::HashMap}};

             static {}: LazyLock<HashMap<{}>> = LazyLock::new(|| {{
                let mut map = HashMap::new();
                {}
                map
            }});",
        name,
        map_sig,
        entries
            .map(|data| format!("map.insert({});", data))
            .collect::<Vec<String>>()
            .join("\n")
    )
}

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

fn posts() -> Result<()> {
    println!("cargo::rerun-if-changed={}", POSTS_DIR);

    let mut parser_opts = Options::empty();
    parser_opts.insert(Options::ENABLE_TABLES);
    parser_opts.insert(Options::ENABLE_STRIKETHROUGH);
    parser_opts.insert(Options::ENABLE_GFM);

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
            let parser = Parser::new_ext(&input, parser_opts);

            let mut html = String::new();
            pulldown_cmark::html::push_html(&mut html, parser);

            let id = escape(name);
            let rev = get_revision(path)?;
            // # abababa\nbabababa -> # abababa -> ababababa
            let mut lines = input.lines();
            let (_, title) = lines
                .next()
                .unwrap_or_default()
                .split_once("# ")
                .unwrap_or_default();
            let summary = lines.skip(1).next().unwrap_or_default();

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

fn assets() -> Result<()> {
    println!("cargo::rerun-if-changed={}", ASSETS_DIR);

    let assets_path = env::current_dir()?.join(ASSETS_DIR);
    let entries = WalkDir::new(assets_path.clone())
        .into_iter()
        .map(|entry| {
            let path = entry?;
            let name = path
                .file_name()
                .to_str()
                .ok_or(eyre!("file name is not utf-8"))?;

            Ok((escape(name), path.path().to_path_buf()))
        })
        .collect::<Result<Vec<(String, PathBuf)>>>()?
        .into_iter()
        .filter(|(_, p)| p.is_file())
        .map(|(name, path)| {
            Ok(format!(
                "{}, {}.to_vec()",
                name,
                Literal::byte_string(&fs::read(path)?).to_string()
            ))
        })
        .collect::<Result<Vec<String>>>()?;

    fs::write(
        Path::new(&env::var("OUT_DIR")?).join("assets.rs"),
        map("ASSETS", "&str, Vec<u8>", entries.into_iter()),
    )?;

    Ok(())
}

fn main() -> Result<()> {
    posts()?;
    assets()?;

    Ok(())
}
