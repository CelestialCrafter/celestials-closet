use std::{env, fs, path::Path};

use eyre::{eyre, Result};
use proc_macro2::Literal;
use pulldown_cmark::{Options, Parser};

const BLOGS_DIR: &str = "blogs";
const ASSETS_DIR: &str = "assets";

fn escape(input: &str) -> String {
    Literal::string(input).to_string()
}

fn map(name: &str, map_sig: &str, entries: impl Iterator<Item = String>) -> String {
    format!(
        "use std::{{sync::LazyLock, collections::HashMap}};

             pub static {}: LazyLock<HashMap<{}>> = LazyLock::new(|| {{
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

fn blogs() -> Result<()> {
    println!("cargo::rerun-if-changed={}", BLOGS_DIR);

    let mut parser_opts = Options::empty();
    parser_opts.insert(Options::ENABLE_TABLES);
    parser_opts.insert(Options::ENABLE_STRIKETHROUGH);
    parser_opts.insert(Options::ENABLE_GFM);

    let entries = fs::read_dir(env::current_dir()?.join(BLOGS_DIR))?
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

            let mut output = String::new();
            pulldown_cmark::html::push_html(&mut output, parser);

            Ok(format!(
                "{}, ({}, {})",
                escape(name),
                escape(path),
                escape(output.as_str())
            ))
        })
        .collect::<Result<Vec<String>>>()?;

    fs::write(
        Path::new(&env::var("OUT_DIR")?).join("blogs.rs"),
        map("BLOGS", "&str, (&str, &str)", entries.into_iter()),
    )?;

    Ok(())
}

fn assets() -> Result<()> {
    println!("cargo::rerun-if-changed={}", ASSETS_DIR);
    let assets_path = env::current_dir()?.join(ASSETS_DIR);
    let entries = fs::read_dir(assets_path.clone())?
        .into_iter()
        .map(|entry| {
            let path = entry?.path();
            let name = path
                .file_name()
                .ok_or(eyre!("no file name on path"))?
                .to_str()
                .ok_or(eyre!("file name is not utf-8"))?;

            Ok(format!(
                "{}, {}.to_vec()",
                escape(name),
                Literal::byte_string(&fs::read(path.clone())?).to_string()
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
    blogs()?;
    assets()?;

    Ok(())
}
