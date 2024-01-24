//! 输出
//!
//! ```powershell
//! $env:RUST_LOG="info"
//! cargo run --bin cli_out_log
//! ```

use log::{info, warn};

fn main() {
    env_logger::init();
    info!("starting up");
    warn!("oops, nothing implemented!");
}
