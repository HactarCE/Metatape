use super::parser::tokens::Instructions;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Program {
    pub subroutines: HashMap<String, Instructions>,
    pub instructions: Instructions,
}

impl Program {
    pub fn new() -> Program {
        Program {
            subroutines: HashMap::new(),
            instructions: vec![],
        }
    }
}
