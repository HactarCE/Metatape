use io::{Read, Write};
use std::io;

pub struct StdInBitBuffer {
    byte: u8,
    bit_idx: u8,
    pub byte_reader: Box<dyn Fn() -> u8>,
}

impl StdInBitBuffer {
    pub fn new() -> Self {
        Self {
            byte: 0,
            bit_idx: 0,
            // If for whatever reason we can't read the byte, use 0.
            byte_reader: Box::new(|| io::stdin().bytes().next().unwrap_or(Ok(0)).unwrap_or(0)),
        }
    }
    pub fn read_bit(&mut self) -> bool {
        if self.bit_idx == 0 {
            self.bit_idx = 8;
            self.byte = (self.byte_reader)();
        }
        self.bit_idx -= 1;
        self.byte & (1 << self.bit_idx) != 0
    }
}

pub struct StdOutBitBuffer {
    byte: u8,
    bit_idx: u8,
    pub byte_writer: Box<dyn Fn(u8)>,
}

impl StdOutBitBuffer {
    pub fn new() -> Self {
        Self {
            byte: 0,
            bit_idx: 8,
            byte_writer: Box::new(|byte| {
                // We don't care whether the write actually succeeds.
                let _ = io::stdout().write(&[byte]);
                let _ = io::stdout().flush();
            }),
        }
    }
    pub fn write_bit(&mut self, bit: bool) {
        self.bit_idx -= 1;
        if bit {
            // Set the bit.
            self.byte |= 1 << self.bit_idx;
        }
        if self.bit_idx == 0 {
            (self.byte_writer)(self.byte);
            self.bit_idx = 8;
            self.byte = 0;
        }
    }
}
