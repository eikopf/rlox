use crate::value::Value;
use thiserror::Error;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Error)]
enum InvalidByteCode {
    #[error("Expected a valid operand; got {0}")]
    UnexpectedOperand(u8),
    #[error("Expected a valid operation; got {0}")]
    InvalidOperation(u8),
}

/// Represents a bytecode operation
#[derive(Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Op {
    Constant, // stores an index into a chunk's constants vector
    Return,
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Op::Constant => "CONSTANT",
            Op::Return => "RETURN",
        })
    }
}

/// Represents a sequence of bytecode operations
#[derive(Debug)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub lines: Vec<usize>, // expected to match the number of opcodes
    pub constants: Vec<Value>,
}

impl Default for Chunk {
    fn default() -> Self {
        Self { 
            code: Default::default(),
            lines: Default::default(),
            constants: Default::default(),
        }
    }
}

impl<'a> IntoIterator for &'a Chunk {
    type Item = Vec<u8>; // yields a list whose head is the operator and tail is the operands

    type IntoIter = ChunkIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ChunkIterator {
            source: self,
            index: 0,
        }
    }
}

impl Chunk {
    pub fn push(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: Value) -> u8 {
        self.constants.push(value);
        return (self.constants.len() - 1) as u8
    }

    pub fn disassemble(&self, name: &str) -> String {
        let mut result = String::from(format!("== {} ==\n", name));

        self.into_iter().enumerate().for_each(|(i, data)| {
            // instruction number
            result.push_str(format!("{:04} ", i).as_str());

            // line number
            if i > 0 && self.lines[i] == self.lines[i-1] {
                result.push_str(format!("{}", "  |  ").as_str());
            } else {
                result.push_str(format!("{:4} ", self.lines[i]).as_str());
            }

            // instruction-specific formatting
            result.push_str(format!("{}\n", match Op::try_from(data[0]).unwrap() {
                Op::Constant => format!(
                    "{:<16} {:4} {}", 
                    Op::Constant, 
                    data[1], 
                    self.constants[data[1] as usize]
                ),

                default @ _ => default.to_string(),
            }).as_str())
        });

        return result
    }
}

pub struct ChunkIterator<'a> {
    source: &'a Chunk,
    index: usize,
}

impl Iterator for ChunkIterator<'_> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.source.code.len() {
            return None
        }

        let mut item = Vec::default();

        match Op::try_from(self.source.code[self.index]) {
            Ok(Op::Return) => {     // arity == 0
                item.push(self.source.code[self.index]);
            },

            Ok(Op::Constant) => {   // arity == 1
                // slices two elements from the source, copies them into a vector, and then
                // passes the vector by mutable reference into Vec::append
                item.append(&mut self.source.code[self.index..=self.index + 1].into());
                self.index += 1;
            },

            Err(_) => panic!("Got bad operand: {:x}", self.source.code[self.index])
        };

        self.index += 1;

        return Some(item)
    }
}
