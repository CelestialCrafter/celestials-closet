mod assets;
mod posts;
mod highlighting;

use std::fmt::Display;

use eyre::Result;
use proc_macro2::Literal;

fn escape(input: &str) -> impl Display {
    Literal::string(input)
}

fn hashmap(name: &str, map_sig: &str, entries: impl Iterator<Item = impl Display>) -> String {
    format!(
        "use std::{{sync::LazyLock, collections::HashMap}};

        static {}: LazyLock<HashMap<{}>> = LazyLock::new(|| HashMap::from([
            {}
        ]));",
        name,
        map_sig,
        entries.map(|v| v.to_string()).collect::<Vec<_>>().join(",\n")
    )
}

fn main() -> Result<()> {
    posts::process_posts()?;
    assets::pack_assets()?;

    Ok(())
}
