// use std::fmt;

use crate::lines::Lines;
use crate::opcode::Byte;
use crate::value::Value;

pub struct Chunk {
    pub code: Vec<Byte>,
    pub lines: Lines,
    pub consts: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::with_capacity(8),
            lines: Lines::new(),
            consts: Vec::with_capacity(8),
        }
    }

    pub fn write(&mut self, byte: Byte, line: i64) {
        self.code.push(byte);
        self.lines.add(line);
    }

    pub fn add_constant(&mut self, value: Value) -> Byte {
        self.consts.push(value);
        (self.consts.len() - 1) as Byte
    }
}
