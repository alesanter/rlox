use crate::chunk::Chunk;
use crate::opcode::{Byte, Op};

impl Chunk {
    pub fn disassemble(&self, name: &str) {
        println!("== {} ==", name);

        let mut offset: usize = 0;
        while let Some(op) = self.code.get(offset) {
            offset = self.disassemble_instruction(offset, *op)
        }

        println!();
    }

    pub fn disassemble_instruction(&self, offset: usize, op: Byte) -> usize {
        print!("{:04} ", offset);
        let line = self.lines.get(offset).unwrap();
        if offset > 0 && line == self.lines.get(offset - 1).unwrap() {
            print!("   | ")
        } else {
            print!("{:4} ", line);
        }

        match op.try_into() {
            Err(_) => panic!("{}", op),
            Ok(op) => match op {
                Op::Return => simple_instruction("OP_RETURN", offset),
                Op::Constant => {
                    let addr = self.code[offset + 1];
                    self.constant_instruction("OP_CONSTANT", addr);
                    return offset + 2;
                }
                Op::Negate => simple_instruction("OP_NEGATE", offset),
                Op::Add => simple_instruction("OP_ADD", offset),
                Op::Subtract => simple_instruction("OP_SUBTRACT", offset),
                Op::Multiply => simple_instruction("OP_MULTIPLY", offset),
                Op::Divide => simple_instruction("OP_DIVIDe", offset),
            },
        }
    }

    fn constant_instruction(&self, name: &str, addr: Byte) {
        println!(
            "{:-16} {:4} '{}'",
            name,
            addr,
            self.consts[addr as usize].to_string()
        );
    }
}

#[inline]
fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    return offset + 1;
}
