use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Script to run
    #[arg(value_name = "FILE")]
    pub script: PathBuf,

    /// Either to print debug traces or not
    #[arg(long, short)]
    pub debug: bool,

    /// Either to print the stack for each instruction
    #[arg(long, short)]
    pub print_stack: bool,
}
