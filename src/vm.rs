use crate::{
    chunk::Chunk,
    opcode::{Byte, Op},
    value::Value,
};

pub struct VM<'a> {
    chunk: Box<&'a Chunk>,
    ip: &'a Vec<Byte>,
    stack: Vec<Value>,
}

#[derive(Debug)]
pub enum Result {
    Ok = 0,
    CompileError,
    RuntimeError,
    UndefinedOpcode,
}

impl<'a> VM<'a> {
    pub fn new<'b>(chunk: &'b Chunk) -> Self
    where
        'b: 'a,
    {
        VM {
            chunk: Box::new(chunk),
            ip: &chunk.code,
            stack: Vec::with_capacity(256),
        }
    }

    pub fn interpret(&mut self) -> Result {
        self.run()
    }

    fn run(&mut self) -> Result {
        let read_byte = |offset| self.ip[offset];

        let read_const = |offset| {
            let addr = read_byte(offset);
            self.chunk.consts[addr as usize]
        };

        let mut offset = 0;
        loop {
            let op = read_byte(offset);
            #[cfg(debug_trace_execution)]
            {
                if !self.stack.is_empty() {
                    print!("        | ");
                    for value in &self.stack {
                        print!("[ {:.3} ]", value.0)
                    }
                    println!();
                }
                self.chunk.disassemble_instruction(offset, op);
            }
            offset += 1;

            match op.try_into() {
                Err(_) => return Result::UndefinedOpcode,
                Ok(op) => match op {
                    Op::Return => {
                        let value = self.pop().0;
                        println!("{:}", value);
                        return Result::Ok;
                    }
                    Op::Constant => {
                        let constant = read_const(offset);
                        offset += 1;
                        self.push(constant);
                    }
                    Op::Negate => self.modify(|v| Value(-v.0)),
                    Op::Add => self.binaryOp(|v, w| Value(v.0 + w.0)),
                    Op::Subtract => self.binaryOp(|v, w| Value(v.0 - w.0)),
                    Op::Multiply => self.binaryOp(|v, w| Value(v.0 * w.0)),
                    Op::Divide => self.binaryOp(|v, w| Value(v.0 / w.0)),
                    // _ => {}
                },
            }
        }
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value)
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().expect("Empty VM stack.")
    }

    fn modify<F>(&mut self, f: F)
    where
        F: FnOnce(Value) -> Value,
    {
        let pos = self.stack.len() - 1;
        let value = self.stack[pos];
        self.stack[pos] = f(value);
    }

    fn binaryOp<F>(&mut self, f: F)
    where
        F: FnOnce(Value, Value) -> Value,
    {
        let b = self.pop();
        let a = self.pop();
        self.push(f(a, b));
    }
}
