use color_eyre::eyre::Result;
use core::runner::Runner;

use clap::Parser;
use cli::cli::Cli;

pub mod cli;
pub mod core;

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli: Cli = Cli::try_parse()?;

    let runner: Runner = Runner::new(cli.script, cli.debug, cli.print_stack);

    runner.run()
}
