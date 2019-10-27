#![allow(dead_code)]

use io::{Read, Write};
use rand::thread_rng;
use rand::RngCore;
use std::io;

use super::program::{Instruction, Instructions, Program};
use super::tape::Head;

const PRINT_STATE: bool = false;

pub struct Runtime<'a> {
    head: Head,
    program: &'a Program<'a>,
    program_stack: Vec<(&'a Instructions, usize, Option<Head>)>,
    /// Buffer of input bits. The lowest bit is at index 0, and the highest bit is at index 7.
    input_buffer: StdInBitBuffer,
    /// Buffor of output bits. The highest bit is at index 0, and the lowest bit is at index 7.
    output_buffer: StdOutBitBuffer,
}

impl<'a> Runtime<'a> {
    pub fn new(program: &'a Program) -> Runtime<'a> {
        Runtime {
            head: Head::new(),
            program: program,
            program_stack: vec![(&program.instructions, 0, None)],
            input_buffer: StdInBitBuffer::new(),
            output_buffer: StdOutBitBuffer::new(),
        }
    }

    pub fn step(&mut self) -> Result<(), RuntimeError> {
        // Fetch the current block.
        if let Some((instructions, instruction_pointer, _)) = self.program_stack.last_mut() {
            let mut go_to_next_instruction = true;
            let (current_instruction_str_idx, current_instruction) = &instructions
                .get(*instruction_pointer)
                .ok_or(RuntimeError::InstructionPointerOutOfBounds)?;
            let old_ip = *instruction_pointer;
            // Fetch the current instruction.
            match current_instruction {
                Instruction::Nop | Instruction::EndIf | Instruction::Loop => (),

                Instruction::Left => self.head = self.head.move_left(),
                Instruction::Right => self.head = self.head.move_right(),
                Instruction::Enter => self.head = self.head.enter(),
                Instruction::Exit => self.head = self.head.exit(),

                Instruction::Null => self.head = self.head.null_child(),

                Instruction::If(destination) => {
                    if !self.head.has_child() {
                        *instruction_pointer = *destination;
                    }
                }
                Instruction::Else(destination) | Instruction::EndLoop(destination) => {
                    *instruction_pointer = *destination;
                }

                Instruction::Block(instruction_block) => {
                    self.program_stack.push((instruction_block, 0, None));
                    go_to_next_instruction = false;
                }
                Instruction::Call(subroutine_name) => {
                    self.program_stack.push((
                        self.program
                            .subroutines
                            .get(subroutine_name)
                            .ok_or(RuntimeError::SubroutineNotFound)?,
                        0,
                        None,
                    ));
                    go_to_next_instruction = false;
                }
                Instruction::Fork(instruction_block) => {
                    self.program_stack
                        .push((instruction_block, 0, Some(self.head.clone())));
                    go_to_next_instruction = false;
                }

                Instruction::Random => {
                    if thread_rng().next_u32() % 2 == 0 {
                        self.head = self.head.null_child();
                    }
                }
                Instruction::Input => {
                    if !self.input_buffer.read_bit() {
                        self.head = self.head.null_child();
                    }
                }
                Instruction::Output => {
                    self.output_buffer.write_bit(self.head.has_child());
                }
                Instruction::Halt => {
                    // Ignore the returned Result because we don't care if we are
                    // currently at the last instruction; we'll just raise an error
                    // next time this function is called.
                    let _ = self.go_to_next_instruction();
                    println!("halt!");
                    return Err(RuntimeError::Halt);
                }
            }
            if PRINT_STATE {
                let (row, col) =
                    pest::Position::new(self.program.source, *current_instruction_str_idx)
                        .unwrap()
                        .line_col();
                println!(
                    "{:>5}:{:<6}{:<14}{:<2}{}",
                    row,
                    col,
                    format!("{:?}", current_instruction),
                    if old_ip != self.program_stack.last().unwrap().1 {
                        "j"
                    } else {
                        ""
                    },
                    self.head,
                );
            }
            if go_to_next_instruction {
                // Ignore the returned Result because we don't care if we are
                // currently at the last instruction; we'll just raise an error
                // next time this function is called.
                let _ = self.go_to_next_instruction();
            }
            Ok(())
        } else {
            Err(RuntimeError::EndOfProgram)
        }
    }

    fn go_to_next_instruction(&mut self) -> Result<(), RuntimeError> {
        loop {
            // Move to the next instruction.
            let (instructions, instruction_pointer, option_old_tape_head) = self
                .program_stack
                .last_mut()
                .ok_or(RuntimeError::EndOfProgram)?;
            // Check whether we are returning from a fork instruction.
            if let Some(old_tape_head) = option_old_tape_head {
                // Copy the child cell from within the fork and use it to
                // overwrite the child cell from outside the fork.
                self.head = old_tape_head.copy_child_from(&self.head);
                *option_old_tape_head = None;
            }
            *instruction_pointer += 1;
            // Check whether there are more instructions in this block.
            if *instruction_pointer < instructions.len() {
                // If there are, then everything is Ok(()).
                return Ok(());
            } else {
                // If there aren't, then pop this block off the stack and try
                // again.
                self.program_stack.pop();
            }
        }
    }
}

struct StdInBitBuffer {
    byte: u8,
    bit_idx: u8,
}

impl StdInBitBuffer {
    fn new() -> Self {
        Self {
            byte: 0,
            bit_idx: 0,
        }
    }
    fn read_bit(&mut self) -> bool {
        if self.bit_idx == 0 {
            self.bit_idx = 8;
            // If for whatever reason we can't read the byte, use 0.
            self.byte = io::stdin().bytes().next().unwrap_or(Ok(0)).unwrap_or(0);
        }
        self.bit_idx -= 1;
        self.byte & (1 << self.bit_idx) != 0
    }
}

struct StdOutBitBuffer {
    byte: u8,
    bit_idx: u8,
}

impl StdOutBitBuffer {
    fn new() -> Self {
        Self {
            byte: 0,
            bit_idx: 8,
        }
    }
    fn write_bit(&mut self, bit: bool) {
        self.bit_idx -= 1;
        if bit {
            // Set the bit.
            self.byte |= 1 << self.bit_idx;
        }
        if self.bit_idx == 0 {
            // We don't care whether the write actually succeeds.
            let _ = io::stdout().write(&[self.byte]);
            let _ = io::stdout().flush();
            self.bit_idx = 8;
            self.byte = 0;
        }
    }
}

pub enum RuntimeError {
    EndOfProgram,
    InstructionPointerOutOfBounds,
    SubroutineNotFound,
    Halt,
}
