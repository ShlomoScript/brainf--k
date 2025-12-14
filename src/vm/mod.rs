mod error;
mod instruction;

use clio::Output;
use std::io::{stdin, Read, Write};

pub use error::RuntimeError;
pub use instruction::Instruction;

pub struct Interpreter {
    source: Vec<Instruction>,
    tape: [u8; 30000],
    pointer: usize,
    index: usize,
    output: Output,
}
impl Interpreter {
    pub fn new(source: Vec<Instruction>, output: Output) -> Self {
        Self {
            source,
            tape: [0; 30000],
            pointer: 0,
            index: 0,
            output,
        }
    }
    pub fn run(&mut self) -> Result<(), RuntimeError> {
        use Instruction::*;
        while let Some(instruction) = self.source.get(self.index) {
            match instruction {
                Increment(n) => {
                    *self.tape.get_mut(self.pointer).unwrap() =
                        self.tape[self.pointer].wrapping_add(*n)
                }
                Decrement(n) => {
                    *self.tape.get_mut(self.pointer).unwrap() =
                        self.tape[self.pointer].wrapping_sub(*n)
                }
                Forward(n) => {
                    let new = self.pointer + *n;
                    if new >= self.tape.len() {
                        return Err(RuntimeError::PointerOverflow { ip: self.index });
                    }
                    self.pointer = new;
                }
                Backward(n) => {
                    if *n > self.pointer {
                        return Err(RuntimeError::PointerUnderflow { ip: self.index });
                    }
                    self.pointer -= *n;
                }
                OpenBracket { jump } => {
                    if self.tape[self.pointer] == 0 {
                        self.index = *jump;
                    }
                }
                CloseBracket { jump } => {
                    if self.tape[self.pointer] != 0 {
                        self.index = *jump;
                    }
                }
                Output => {
                    self.output.write_all(&[self.tape[self.pointer]])?;
                }
                Input => {
                    let mut buf = [0u8; 1];
                    stdin().read_exact(&mut buf)?;
                    self.tape[self.pointer] = buf[0];
                }
            }
            self.index += 1;
        }
        self.output.flush()?;
        Ok(())
    }
}
