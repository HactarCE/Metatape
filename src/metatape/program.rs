use std::collections::HashMap;
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

// impl fmt::Debug for Instruction {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self{

//         }
//     }
// }
