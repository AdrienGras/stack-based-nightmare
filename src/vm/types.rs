use color_eyre::eyre::Result;

use crate::vm::{instruction::Instruction, vm::VM};

pub type NativeFn = fn(&mut VM, &[Instruction], &mut usize) -> Result<()>;
