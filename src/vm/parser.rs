use color_eyre::eyre::Result;
use tracing::trace;

use crate::vm::instruction::Instruction;

pub struct Parser;

impl Parser {
    #[tracing::instrument(skip_all, level = "trace")]
    pub fn parse_program(lines: &[&str]) -> Result<Vec<Instruction>> {
        let mut instructions = Vec::new();

        for (number, line) in lines.iter().enumerate() {
            trace!("Parsing line {}: {}", number, line);

            let tokens: Vec<&str> = line.trim().split_whitespace().collect();
            trace!("Tokens for line {}: {:?}", number, tokens);

            if tokens.is_empty() {
                trace!("Skipping empty line {}", number);
                continue;
            }

            if tokens[0].starts_with('#') {
                trace!("Skipping comment line {}", number);
                continue;
            }

            trace!("Adding instruction for line {}: {:?}", number, tokens[0]);
            instructions.push(Instruction::Word(tokens[0].to_string()));

            for arg in &tokens[1..] {
                trace!("Adding argument for line {}: {}", number, arg);
                instructions.push(Instruction::Arg(arg.to_string()));
            }
        }

        Ok(instructions)
    }
}
