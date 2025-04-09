use color_eyre::eyre::{Result, eyre};
use std::path::PathBuf;

pub struct Runner {
    pub script_path: PathBuf,
    pub use_debug: bool,
    pub use_print_stack: bool,
}

impl Runner {
    pub fn new(script_path: PathBuf, use_debug: bool, use_print_stack: bool) -> Self {
        Runner {
            script_path,
            use_debug,
            use_print_stack,
        }
    }
    pub fn run(&self) -> Result<()> {
        if !self.script_path.exists() {
            return Err(eyre!("Script file does not exist: {:?}", self.script_path));
        }

        Ok(())
    }
}
