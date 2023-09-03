mod bytecode;
mod value;

use bytecode::{
    Op,
    Chunk,
};

use crate::value::Value;

fn main() {
    let mut chunk = Chunk::default();
    chunk.push(Op::Return as u8, 123);
    let index = chunk.add_constant(1.6 as Value);
    chunk.push(Op::Constant as u8, 123);
    chunk.code.push(index); // we expect that constants do not correspond to a line number
    println!("{}", chunk.disassemble("test chunk"));
    println!("{:?}", chunk);
}
