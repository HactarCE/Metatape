#![allow(dead_code)]

mod io;

use rand::thread_rng;
use rand::RngCore;
use std::mem;

use super::program::{Instruction, InstructionBlock, Program};
use super::tape::Head;

pub struct Runtime {
    program: Program,
    head: Head,
    executing_block: InstructionBlock,
    instruction_pointer: usize,
    call_stack: Vec<Box<dyn FnOnce(&mut Runtime)>>,
    /// Buffer of input bits. The lowest bit is at index 0, and the highest bit is at index 7.
    input_buffer: io::StdInBitBuffer,
    /// Buffor of output bits. The highest bit is at index 0, and the lowest bit is at index 7.
    output_buffer: io::StdOutBitBuffer,
}

impl Runtime {
    pub(super) fn new(program: Program) -> Self {
        let executing_block = program.instructions.clone();
        Self {
            program,
            head: Head::new(),
            executing_block,
            instruction_pointer: 0,
            call_stack: vec![],
            input_buffer: io::StdInBitBuffer::new(),
            output_buffer: io::StdOutBitBuffer::new(),
        }
    }

    pub fn get_program(&self) -> &Program {
        &self.program
    }

    pub fn get_head(&self) -> &Head {
        &self.head
    }

    pub fn get_executing_block(&self) -> &InstructionBlock {
        &self.executing_block
    }

    pub fn get_instruction_pointer(&self) -> usize {
        self.instruction_pointer
    }

    pub fn step(&mut self) -> Result<ExecDebugInfo, RuntimeError> {
        // Fetch the current block.
        let (_current_instruction_str_idx, current_instruction) = self.fetch_instruction()?;
        let mut exec_debug_info = ExecDebugInfo { bit: None };
        // let old_exec_block = self.executing_block;
        // let old_ip = self.instruction_pointer;
        // let new_stack_entry: Option<Box<dyn FnMut(&mut Runtime)>> = None;
        // let new_executing_block: Option<&InstructionBlock> = None;
        struct Call {
            new_executing_block: Option<InstructionBlock>,
            head_restore_function: Option<Box<dyn FnOnce(&Head, &Head) -> Head>>,
        }

        let mut call: Call = Call {
            new_executing_block: None,
            head_restore_function: None,
        };
        // Fetch the current instruction.
        match current_instruction {
            Instruction::Nop | Instruction::EndIf | Instruction::Loop => (),

            Instruction::Left => self.head = self.head.move_left(),
            Instruction::Right => self.head = self.head.move_right(),
            Instruction::Enter => self.head = self.head.enter(),
            Instruction::Exit => self.head = self.head.exit(),

            Instruction::Null => self.head = self.head.null_child(),

            Instruction::If(destination) => {
                if self.head.has_child() {
                    exec_debug_info.bit = Some(true);
                } else {
                    exec_debug_info.bit = Some(false);
                    self.instruction_pointer = *destination;
                }
            }
            Instruction::Else(destination) | Instruction::EndLoop(destination) => {
                self.instruction_pointer = *destination;
            }

            Instruction::Block(instruction_block) => {
                call.new_executing_block = Some(instruction_block.clone());
            }

            Instruction::Call(subroutine_name) => {
                call.new_executing_block = Some(
                    self.program
                        .subroutines
                        .get(subroutine_name)
                        .ok_or(RuntimeError::SubroutineNotFound)?
                        .clone(),
                );
            }
            Instruction::Fork(instruction_block) => {
                call.new_executing_block = Some(instruction_block.clone());
                call.head_restore_function = Some(Box::new(|old_head, new_head| {
                    old_head.copy_child_from(&new_head)
                }));
            }

            Instruction::Random => {
                if thread_rng().next_u32() % 2 == 1 {
                    exec_debug_info.bit = Some(true);
                } else {
                    exec_debug_info.bit = Some(false);
                    self.head = self.head.null_child();
                }
            }
            Instruction::Input => {
                if self.input_buffer.read_bit() {
                    exec_debug_info.bit = Some(true);
                } else {
                    exec_debug_info.bit = Some(false);
                    self.head = self.head.null_child();
                }
            }
            Instruction::Output => {
                let bit = self.head.has_child();
                exec_debug_info.bit = Some(bit);
                self.output_buffer.write_bit(bit);
            }
            Instruction::Halt => {
                // Ignore the returned Result because we don't care if we are
                // currently at the last instruction; we'll just raise an error
                // next time this function is called.
                let _ = self.go_to_next_instruction();
                println!("halt!");
                return Err(RuntimeError::Halt);
                // TODO improve this behavior
            }
        }
        if let Call {
            new_executing_block: None,
            head_restore_function: None,
        } = call
        {
            // Ignore the returned Result because we don't care if we are
            // currently at the last instruction; we'll just raise an error
            // next time this function is called.
            let _ = self.go_to_next_instruction();
        } else {
            let exec_block_fn: Box<dyn FnOnce(&mut Runtime)> = match call.new_executing_block {
                None => Box::new(|_| ()),
                Some(new_executing_block) => {
                    let old_index = mem::replace(&mut self.instruction_pointer, 0);
                    let old_executing_block =
                        mem::replace(&mut self.executing_block, new_executing_block);
                    Box::new(move |runtime: &mut Runtime| {
                        runtime.instruction_pointer = old_index;
                        runtime.executing_block = old_executing_block;
                    })
                }
            };
            let head_fn: Box<dyn FnOnce(&mut Runtime)> = match call.head_restore_function {
                None => Box::new(move |_| ()),
                Some(restore_head) => {
                    let old_head = self.head.clone();
                    Box::new(move |runtime: &mut Runtime| {
                        runtime.head = restore_head(&old_head, &runtime.head)
                    })
                }
            };
            self.call_stack.push(Box::new(|runtime: &mut Runtime| {
                exec_block_fn(runtime);
                head_fn(runtime);
            }));
        }
        Ok(exec_debug_info)
    }

    pub fn fetch_instruction(&self) -> Result<&(usize, Instruction), RuntimeError> {
        self.executing_block
            .get(self.instruction_pointer)
            .ok_or(RuntimeError::InstructionPointerOutOfBounds)
    }

    fn go_to_next_instruction(&mut self) -> Result<(), RuntimeError> {
        loop {
            // Move to the next instruction.
            self.instruction_pointer += 1;
            // If we have reached the end of this block ...
            if self.instruction_pointer >= self.executing_block.len() {
                // ... then pop one off the stack.
                self.call_stack.pop().ok_or(RuntimeError::EndOfProgram)?(self);
            } else {
                return Ok(());
            }
        }
    }
}

pub struct ExecDebugInfo {
    pub bit: Option<bool>,
}

#[derive(Debug)]
pub enum RuntimeError {
    EndOfProgram,
    InstructionPointerOutOfBounds,
    SubroutineNotFound,
    Halt,
}
