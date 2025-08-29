use color_eyre::eyre::{OptionExt, eyre};
use tracing::debug;

use crate::vm::{builtins::Builtin, instruction::Instruction, types::NativeFn};

pub struct Push;
pub struct Add;
pub struct Print;

impl Builtin for Push {
    fn name(&self) -> &str {
        "PUSH"
    }

    #[tracing::instrument(skip_all, level = "trace")]
    fn function(&self) -> NativeFn {
        |vm, program, pc| {
            debug!("Executing PUSH");
            // push counter to get value for push
            *pc += 1;

            if let Some(Instruction::Arg(arg)) = program.get(*pc) {
                let value = arg.parse::<i64>()?;

                debug!("Pushing value onto stack: {}", value);
                vm.stack.push(value);
            } else {
                return Err(eyre!("Expected argument after PUSH"));
            }

            Ok(())
        }
    }
}

impl Builtin for Add {
    fn name(&self) -> &str {
        "ADD"
    }

    #[tracing::instrument(skip_all, level = "trace")]
    fn function(&self) -> NativeFn {
        |vm, _, _| {
            debug!("Executing ADD");

            let b = vm.stack.pop().ok_or_eyre("Stack underflow")?;

            let a = vm.stack.pop().ok_or_eyre("Stack underflow")?;

            debug!("Popping values from stack: {} + {}", a, b);
            debug!("Pushing result onto stack: {}", a + b);
            vm.stack.push(a + b);

            Ok(())
        }
    }
}

impl Builtin for Print {
    fn name(&self) -> &str {
        "PRINT"
    }

    fn function(&self) -> NativeFn {
        |vm, _, _| {
            debug!("Executing PRINT");
            let value = vm.stack.pop().ok_or_eyre("Stack underflow")?;

            println!("{}", value);

            Ok(())
        }
    }
}
