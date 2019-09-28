use crate::metatape::program::Program;
use crate::metatape::tape::Head;

pub struct Runtime {
    head: Head,
    program: Program,
}

impl Runtime {
    pub fn new() -> Runtime {
        Runtime {
            head: Head::new(),
            program: Program::new(),
        }
    }
}
