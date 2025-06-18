use std::{
    env, fs,
    path::Path,
};

use eyre::{eyre, ErrReport, Result};
use proc_macro2::Literal;

use crate::{escape, hashmap};

const ASSETS_DIR: &str = "assets";

pub fn pack_assets() -> Result<()> {
    println!("cargo::rerun-if-changed={}", ASSETS_DIR);

    let assets_path = env::current_dir()?.join(ASSETS_DIR);
    let entries = fs::read_dir(assets_path.clone())?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .map(|path| {
            let name = path
                .file_name()
                .ok_or(eyre!("path does not have file name"))?
                .to_str()
                .ok_or(eyre!("file name is not utf-8"))?;

            Ok(format!(
                "({}, {}.as_slice())",
                escape(name),
                Literal::byte_string(&fs::read(path)?)
            ))
        })
        .map(|v| {
            v.inspect_err(|err: &ErrReport| println!("cargo:warning=could not process post: {err}"))
        }).collect::<Result<Vec<_>>>().map_err(|_| eyre!("could not process assets"))?;

    fs::write(
        Path::new(&env::var("OUT_DIR")?).join("assets.rs"),
        hashmap("ASSETS", "&str, &[u8]", entries.into_iter()),
    )?;

    Ok(())
}
