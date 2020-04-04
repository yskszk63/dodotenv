#![warn(clippy::all)]

//! # dodotenv: Load .env and run program.
//!
//! ## Usage
//! ```
//! dodotenv <program> [args]...
//! ```

use std::process::Command;
use std::os::unix::process::CommandExt as _;

use structopt::StructOpt;
use structopt::clap::AppSettings::AllowLeadingHyphen;

/// Load .env and run program.
#[derive(StructOpt, Debug)]
#[structopt(setting=AllowLeadingHyphen)]
struct Opt {
    /// Program to run with .env
    program: String,

    /// Program arguments
    args: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let Opt {program, args} = Opt::from_args();
    if let Err(e) = dotenv::dotenv() {
        eprintln!("[{}] WARNING: Failed to load .env {}", env!("CARGO_PKG_NAME"), e);
    };

    let err = Command::new(program).args(args).exec();

    Err(err)?
}
