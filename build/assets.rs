use std::{
    env, fs,
    path::{Path, PathBuf}
};

use eyre::{eyre, Result};
use proc_macro2::Literal;
use walkdir::WalkDir;

use crate::{escape, map};

const ASSETS_DIR: &str = "assets";

pub fn pack_assets() -> Result<()> {
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
