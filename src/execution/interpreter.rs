use std::io::{self, Read, Write};

use crate::syntax::Instruction;

#[derive(Debug)]
pub struct Interpreter {
    tape: Vec<u8>,
    pointer: usize,
}

impl Interpreter {
    pub fn new(tape_size: usize) -> Self {
        Interpreter {
            tape: vec![0; tape_size],
            pointer: 0,
        }
    }

    pub fn interpret(&mut self, instructions: &[Instruction]) {
        for instruction in instructions {
            match instruction {
                Instruction::Increment { value } => {
                    self.tape[self.pointer] = self.tape[self.pointer].wrapping_add_signed(value.0);
                }
                Instruction::CellIncrement { value } => {
                    self.pointer = (self.pointer as isize + *value as isize) as usize;
                }
                Instruction::Loop { nodes } => {
                    while self.tape[self.pointer] != 0 {
                        self.interpret(nodes);
                    }
                }
                Instruction::Write => {
                    print!("{}", self.tape[self.pointer] as char);
                    io::stdout().flush().unwrap();
                }
                Instruction::Read => {
                    let mut buffer = [0];
                    io::stdin().read_exact(&mut buffer).unwrap();
                    self.tape[self.pointer] = buffer[0];
                }
                Instruction::Set { value } => {
                    self.tape[self.pointer] = *value;
                }
            }
        }
    }
}
