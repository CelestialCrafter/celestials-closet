use std::sync::LazyLock;

use eyre::Result;
use pico_args::Arguments;

pub struct Args {
    pub port: u16,
    pub log: String,
}

pub static ARGS: LazyLock<Args> =
    LazyLock::new(|| parse_args().expect("could not parse arguments"));

fn parse_args() -> Result<Args> {
    let mut args = Arguments::from_env();

    Ok(Args {
        port: args.opt_value_from_str("--port")?.unwrap_or(80),
        log: args.opt_value_from_str("--log")?.unwrap_or("info".to_string()),
    })
}
