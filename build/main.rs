mod assets;
mod posts;
mod highlighting;

use eyre::Result;
use proc_macro2::Literal;

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

fn main() -> Result<()> {
    posts::process_posts()?;
    assets::pack_assets()?;

    Ok(())
}
