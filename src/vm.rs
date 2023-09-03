use crate::{bytecode::{Chunk, Op}, value::Value};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InterpreterError {
    #[error("An error has occurred at compile-time.")]
    AtCompileTime,
    #[error("An error has occurred at runtime.")]
    AtRunTime,
}

pub struct VM<'a> {
    ip: *const u8,
    chunk: &'a Chunk,
}

impl<'a> From<&'a Chunk> for VM<'a> {
    fn from(value: &'a Chunk) -> Self {
        VM { chunk: value, ip: value.code.as_ptr() }
    }
}

impl<'a> VM<'a> {
    pub fn interpret(&mut self, chunk: &'a Chunk) -> Result<(), InterpreterError> {
        self.chunk = chunk;
        self.ip = chunk.code.as_ptr();
        self.run()
    } 

    unsafe fn advance_and_deref_ip(&mut self) -> u8 {
        self.ip = self.ip.wrapping_add(1);
        return unsafe { *self.ip };
    }

    fn run(&mut self) -> Result<(), InterpreterError> {
        loop {
           match unsafe {
                Op::try_from(self.advance_and_deref_ip()) // deref and process
           } {
                Ok(Op::Return) => return Ok(()),
                Ok(Op::Constant) => {
                        let index: u8 = unsafe { self.advance_and_deref_ip() };
                        let constant: Value = self.chunk.constants[index as usize];
                        println!("{}\n", constant);
                        return Ok(());
                },
                Err(_) => return Err(InterpreterError::AtRunTime),
           }
        }
    }
}
