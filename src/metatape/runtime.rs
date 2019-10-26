#![allow(dead_code)]

use super::program::Program;
use super::tape::Head;

pub struct Runtime<'a> {
    head: Head,
    program: &'a Program<'a>,
}

impl<'a> Runtime<'a> {
    pub fn new(program: &'a Program) -> Runtime<'a> {
        Runtime {
            head: Head::new(),
            program: program,
        }
    }
}
