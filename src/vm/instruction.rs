#[derive(Debug, Clone)]
pub enum Instruction {
    Word(String),
    Arg(String),
}
