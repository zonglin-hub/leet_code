//! 更好地反馈错误
//!
//! ```
//! cargo run --example error
//! ```

#![allow(unused)]

use anyhow::{Context, Result};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    pattern: String,

    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let path = "text.txt";

    let context =
        std::fs::read_to_string(path).with_context(|| format!("could not read file `{}`", path))?;

    println!("file context: \n{}", context);
    Ok(())
}
