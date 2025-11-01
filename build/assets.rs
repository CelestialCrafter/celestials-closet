use std::{
    env, fs,
    io::{Cursor, Read},
    path::Path,
};

use eyre::{ErrReport, Result, eyre};
use image::{ExtendedColorType, ImageReader, codecs::webp::WebPEncoder, imageops::FilterType};
use proc_macro2::Literal;

use crate::{escape, hashmap};

const ASSETS_DIR: &str = "assets";

const IMAGE_SIZE: u32 = 192;

pub fn pack_assets() -> Result<()> {
    println!("cargo::rerun-if-changed={}", ASSETS_DIR);

    let estimate =
        (ExtendedColorType::Rgba8.bits_per_pixel() as u32 * IMAGE_SIZE * IMAGE_SIZE) as usize;
    let mut buffer = Vec::with_capacity(estimate);

    let assets_path = env::current_dir()?.join(ASSETS_DIR);
    let entries = fs::read_dir(assets_path.clone())?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .map(|mut path| {
            buffer.clear();
            fs::File::open(&path)?.read_to_end(&mut buffer)?;

            let opt_level = match path
                .extension()
                .and_then(|os_str| os_str.to_str())
                .and_then(|extension| extension.strip_prefix("opt-"))
            {
                Some(level) => level
                    .parse()
                    .map_err(|err| eyre!("could not parse optimization level: {err}")),
                None => Err(eyre!("asset did not have optimization level")),
            }?;
            path.set_extension("");

            match opt_level {
                0 => (),
                1 | 2 => {
                    let mut img = ImageReader::new(Cursor::new(&buffer))
                        .with_guessed_format()?
                        .decode()?;
                    if opt_level == 2 {
                        img = img.resize(IMAGE_SIZE, IMAGE_SIZE, FilterType::Lanczos3);
                    }

                    buffer.clear();
                    img.write_with_encoder(WebPEncoder::new_lossless(&mut buffer))?;

                    path.set_extension("webp");
                }
                _ => return Err(eyre!("optimization level {opt_level} is unsupported")),
            }

            let name = path
                .file_name()
                .ok_or(eyre!("path does not have file name"))?
                .to_str()
                .ok_or(eyre!("file name is not utf-8"))?;

            Ok(format!(
                "({}, {}.as_slice())",
                escape(name),
                Literal::byte_string(&buffer)
            ))
        })
        .map(|v| {
            v.inspect_err(|err: &ErrReport| println!("cargo:warning=could not process post: {err}"))
        })
        .collect::<Result<Vec<_>>>()
        .map_err(|_| eyre!("could not process assets"))?;

    fs::write(
        Path::new(&env::var("OUT_DIR")?).join("assets.rs"),
        hashmap("ASSETS", "&str, &[u8]", entries.into_iter()),
    )?;

    Ok(())
}
