use std::sync::LazyLock;

use eyre::Result;
use pico_args::Arguments;

pub struct Args {
    pub port: u16,
}

pub static ARGS: LazyLock<Args> =
    LazyLock::new(|| parse_args().expect("could not parse arguments"));

fn parse_args() -> Result<Args> {
    let mut args = Arguments::from_env();

    Ok(Args {
        port: args.opt_value_from_str("--port")?.unwrap_or(8080),
    })
}
