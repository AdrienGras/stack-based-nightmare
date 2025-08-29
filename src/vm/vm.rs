use std::collections::HashMap;

use color_eyre::eyre::{Result, eyre};
use tracing::debug;

use crate::vm::{instruction::Instruction, types::NativeFn};

pub struct VM {
    pub stack: Vec<i64>,
    pub native_words: HashMap<String, NativeFn>,
}

impl VM {
    #[tracing::instrument(skip_all, level = "trace")]
    pub fn run(&mut self, program: &[Instruction]) -> Result<()> {
        debug!("Starting VM execution");
        debug!("Initial stack state: {:?}", self.stack);

        let mut pc = 0;

        while pc < program.len() {
            debug!("Executing instruction at pc {}: {:?}", pc, program[pc]);

            match &program[pc] {
                Instruction::Word(name) => {
                    debug!("Found word instruction: {}", name);

                    if let Some(handler) = self.native_words.get(name) {
                        debug!("Found native handler for {}: {:?}", name, handler);
                        handler(self, program, &mut pc)?;
                    } else {
                        return Err(eyre!("Unknown instruction: {} at pc {}", name, pc));
                    }
                }
                Instruction::Arg(_) => {
                    return Err(eyre!("Unexpected argument: {:?} at pc {}", program[pc], pc));
                }
            }
            debug!(
                "Stack state after instruction at pc {}: {:?}",
                pc, self.stack
            );

            debug!("Advancing program counter from {} to {}", pc, pc + 1);
            pc += 1;
        }

        Ok(())
    }
}
