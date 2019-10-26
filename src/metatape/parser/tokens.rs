pub type Instructions = Vec<(usize, Instruction)>;

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
    Load(String),

    Fork(Box<Instructions>),
}
