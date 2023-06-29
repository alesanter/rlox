mod chunk;
mod debug;
mod lines;
mod opcode;
mod value;
mod vm;

use crate::chunk::Chunk;
use crate::opcode::Op;
use crate::value::Value;
use crate::vm::VM;

fn main() {
    let mut chunk = Chunk::new();

    let constant = chunk.add_constant(Value(1.2));
    chunk.write(Op::Constant.into(), 1);
    chunk.write(constant, 1);

    let constant = chunk.add_constant(Value(4.8));
    chunk.write(Op::Constant.into(), 1);
    chunk.write(constant, 1);

    chunk.write(Op::Add.into(), 1);

    let constant = chunk.add_constant(Value(5.6));
    chunk.write(Op::Constant.into(), 1);
    chunk.write(constant, 1);

    chunk.write(Op::Divide.into(), 1);

    chunk.write(Op::Negate.into(), 1);
    chunk.write(Op::Return.into(), 2);

    chunk.disassemble("test");

    let mut vm = VM::new(&chunk);
    vm.interpret();
}
