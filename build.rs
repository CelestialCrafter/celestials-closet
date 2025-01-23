use std::{env, fs, path::Path};

use eyre::{eyre, Result};
use proc_macro2::Literal;
use pulldown_cmark::{Options, Parser};

const BLOGS_DIR: &str = "blogs";

fn escape(input: &str) -> String {
    Literal::string(input).to_string()
}

fn main() -> Result<()> {
    println!("cargo::rerun-if-changed={}", BLOGS_DIR);

    let mut parser_opts = Options::empty();
    parser_opts.insert(Options::ENABLE_TABLES);
    parser_opts.insert(Options::ENABLE_STRIKETHROUGH);
    parser_opts.insert(Options::ENABLE_GFM);

    let mut map_entries = vec![];
    for blog in fs::read_dir(env::current_dir()?.join(BLOGS_DIR))? {
        let blog = blog?.path();

        let input = fs::read_to_string(blog.clone())?;
        let parser = Parser::new_ext(&input, parser_opts);

        let mut output = String::new();
        pulldown_cmark::html::push_html(&mut output, parser);

        let name = blog
            .file_stem()
            .ok_or(eyre!("no stem on path"))?
            .to_str()
            .ok_or(eyre!("file stem not utf-8"))?;

        let path = blog.to_str().ok_or(eyre!("path not utf-8"))?;

        map_entries.push(format!(
            "map.insert({}, ({}, {}));",
            escape(name),
            escape(path),
            escape(output.as_str())
        ));
    }

    let out_dir = env::var("OUT_DIR")?;
    fs::write(
        Path::new(&out_dir).join("blogs.rs"),
        format!(
            "use std::{{sync::LazyLock, collections::HashMap}};

            static BLOGS: LazyLock<HashMap<&str, (&str, &str)>> = LazyLock::new(|| {{
                let mut map = HashMap::new();
                {}
                map
            }});",
            map_entries.join("\n")
        ),
    )?;

    Ok(())
}
