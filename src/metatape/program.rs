use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

pub type InstructionSeq = Vec<(usize, Instruction)>;
pub type InstructionBlock = Rc<InstructionSeq>;
pub type Subroutines = HashMap<String, InstructionBlock>;

#[derive(Debug)]
pub struct Program {
    pub source: String,
    pub subroutines: Subroutines,
    pub instructions: InstructionBlock,
}

#[derive(Debug)]
pub enum Instruction {
    Nop,

    Left,
    Right,
    Enter,
    Exit,
    Null,

    If(usize),
    Else(usize),
    EndIf,

    Loop,
    EndLoop(usize),

    Block(InstructionBlock),

    Random,

    Input,
    Output,

    Halt,

    // IOMode(IOMode),
    // Seek(String),
    Call(String),
    // Load(String),
    Fork(InstructionBlock),
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Use custom formatting to avoid printing every single instruction
            // in a block or fork instruction.
            Self::Block(_) => f.write_str("Block(...)"),
            Self::Fork(_) => f.write_str("Fork(...)"),
            // Use debug formatting for all the rest.
            _ => f.write_str(&format!("{:?}", self)),
        }
    }
}
