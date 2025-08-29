use color_eyre::eyre::{Result, eyre};
use std::path::PathBuf;
use tracing::{debug, trace};

use crate::vm::{builtins::init_builtins, parser::Parser, vm::VM};

#[derive(Debug)]
pub struct Runner {
    pub script_path: PathBuf,
    pub use_debug: bool,
    pub use_print_stack: bool,
}

impl Runner {
    #[tracing::instrument(skip_all, level = "trace")]
    pub fn new(script_path: PathBuf, use_debug: bool, use_print_stack: bool) -> Self {
        debug!("Creating new Runner instance");
        trace!(
            ?script_path,
            use_debug, use_print_stack, "Runner configuration"
        );
        Runner {
            script_path,
            use_debug,
            use_print_stack,
        }
    }

    #[tracing::instrument(skip_all, level = "trace")]
    pub fn run(&self) -> Result<()> {
        debug!("Running script: {:?}", self.script_path);

        if !self.script_path.exists() {
            return Err(eyre!("Script file does not exist: {:?}", self.script_path));
        }

        debug!("Script file exists: {:?}", self.script_path);

        let src = std::fs::read_to_string(&self.script_path)
            .map_err(|e| eyre!("Failed to read script file: {}", e))?;

        debug!("Read script file successfully: {:#?}", self.script_path);

        let lines: Vec<&str> = src.lines().collect();

        debug!("Parsing program...");
        let program = Parser::parse_program(&lines)?;
        debug!("Parsed program successfully: {:#?}", program);

        debug!("Initializing VM...");
        let mut vm = VM {
            stack: Vec::new(),
            native_words: init_builtins(),
        };

        debug!("Running program in VM...");
        vm.run(&program)?;
        debug!("Program executed successfully");

        Ok(())
    }
}
