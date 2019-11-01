use io::{Read, Write};
use std::io;

pub struct StdInBitBuffer {
    byte: u8,
    bit_idx: u8,
}

impl StdInBitBuffer {
    pub fn new() -> Self {
        Self {
            byte: 0,
            bit_idx: 0,
        }
    }
    pub fn read_bit(&mut self) -> bool {
        if self.bit_idx == 0 {
            self.bit_idx = 8;
            // If for whatever reason we can't read the byte, use 0.
            self.byte = io::stdin().bytes().next().unwrap_or(Ok(0)).unwrap_or(0);
        }
        self.bit_idx -= 1;
        self.byte & (1 << self.bit_idx) != 0
    }
}

pub struct StdOutBitBuffer {
    byte: u8,
    bit_idx: u8,
}

impl StdOutBitBuffer {
    pub fn new() -> Self {
        Self {
            byte: 0,
            bit_idx: 8,
        }
    }
    pub fn write_bit(&mut self, bit: bool) {
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
