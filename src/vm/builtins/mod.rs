use std::collections::HashMap;

pub mod builtin_functions;

use builtin_functions::*;
use tracing::debug;

use crate::vm::types::NativeFn;

pub trait Builtin {
    fn name(&self) -> &str;
    fn function(&self) -> NativeFn;
}

#[tracing::instrument(skip_all, level = "trace")]
pub fn init_builtins() -> HashMap<String, NativeFn> {
    let mut map = HashMap::new();

    map.insert(Push.name().to_string(), Push.function());
    map.insert(Add.name().to_string(), Add.function());
    map.insert(Print.name().to_string(), Print.function());

    debug!("Built-in functions initialized: {:#?}", map.keys());

    map
}
