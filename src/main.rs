use color_eyre::eyre::Result;
use core::runner::Runner;
use tracing::{debug, info};

use clap::Parser;
use cli::cli::Cli;

pub mod cli;
pub mod core;
pub mod vm;

#[tokio::main]
#[tracing::instrument(skip_all, level = "trace")]
async fn main() -> Result<()> {
    core::init::init().await?;

    info!("Starting CLI args parsing...");
    let cli: Cli = Cli::try_parse()?;
    debug!("CLI args parsed successfully");

    info!("Creating runner instance...");
    let runner: Runner = Runner::new(cli.script, cli.debug, cli.print_stack);
    debug!("Runner instance created successfully");

    info!("Running script...");
    runner.run()?;

    Ok(())
}
