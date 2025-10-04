mod macros;
mod requests;
mod server;
mod types;
mod utils;

use std::path::PathBuf;

pub use anyhow::Context as ErrorContext;
pub use anyhow::Result;
pub use once_cell;

use crate::server::ServerConfig;
use crate::utils::vfs;

fn main() -> Result<()> {
    let root = PathBuf::from("");
    let config: ServerConfig = vfs::read_config(&root.join("config.json"))?;
    config.build(&root).run()?;
    Ok(())
}
