use std::collections::HashMap;

pub type Instructions = Vec<(usize, Instruction)>;
pub type Subroutines = HashMap<String, Instructions>;

#[derive(Debug)]
pub struct Program<'a> {
    pub source: &'a str,
    pub subroutines: Subroutines,
    pub instructions: Instructions,
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

    Block(Box<Instructions>),

    Random,

    Input,
    Output,

    Halt,

    // IOMode(IOMode),
    // Seek(String),
    Call(String),
    // Load(String),
    Fork(Box<Instructions>),
}

// impl fmt::Debug for Instruction {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self{

//         }
//     }
// }
