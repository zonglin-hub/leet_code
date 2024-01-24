//! grrs 的第一个实现
//!
//! ```sh
//! # 无参数运行时
//! cargo run --bin cli_parsing
//!
//! # 可以直接在 cargo run 后添加 --，并在其后跟上参数来传递
//! cargo run --bin cli_parsing -- main .\examples\cli_parsing.rs
//! ```
//!

#![allow(unused)]

use anyhow::Result;
use structopt::StructOpt;

/// 在文件中搜索模式，并显示包含该模式的行。
#[derive(StructOpt)]
struct Cli {
    /// 要寻找的模式
    pattern: String,

    /// 要读取的文件的路径
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");
    find_matches(&content, &args.pattern, &mut std::io::stdout());
    Ok(())
}

fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line);
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::find_matches;

    #[test]
    fn find_a_match() {
        let mut result = Vec::new();
        find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
        assert_eq!(result, b"lorem ipsum\n");
    }
}
