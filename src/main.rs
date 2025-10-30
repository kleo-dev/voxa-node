#![allow(dead_code)]
mod macros;
mod requests;
mod server;
mod types;
mod utils;

use std::env;
use std::path::PathBuf;

pub use anyhow::Context as ErrorContext;
pub use anyhow::Result;
pub use once_cell;

use crate::server::ServerConfig;
use crate::utils::vfs;

fn main() -> Result<()> {
    let root = PathBuf::from("");

    let config: ServerConfig = if let Ok(env_config) = env::var("VX_CONFIG") {
        serde_json::from_str(&env_config)?
    } else {
        vfs::read_config(&root.join("config.json"))?
    };

    config.build(&root).run()?;

    Ok(())
}
