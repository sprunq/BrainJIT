use crate::syntax::Instruction;
use std::io::{Read, Write};

pub struct Interpreter<'a> {
    input: Box<dyn Read + 'a>,
    output: Box<dyn Write + 'a>,
    tape: Box<[u8]>,
    pointer: usize,
}

impl<'a> Interpreter<'a> {
    pub fn new(input: Box<dyn Read + 'a>, output: Box<dyn Write + 'a>, tape_size: usize) -> Self {
        Interpreter {
            input,
            output,
            tape: vec![0; tape_size].into_boxed_slice(),
            pointer: 0,
        }
    }

    pub fn interpret(&mut self, instructions: &[Instruction]) {
        for instruction in instructions {
            match instruction {
                Instruction::Add { value } => {
                    self.tape[self.pointer] = self.tape[self.pointer].wrapping_add_signed(value.0);
                }
                Instruction::Move { value } => {
                    self.pointer = (self.pointer as isize + *value as isize) as usize;
                }
                Instruction::Loop { nodes } => {
                    while self.tape[self.pointer] != 0 {
                        self.interpret(nodes);
                    }
                }
                Instruction::Write => {
                    self.output.write_all(&[self.tape[self.pointer]]).unwrap();
                }
                Instruction::Read => {
                    let mut buffer = [0];
                    self.input.read_exact(&mut buffer).unwrap();
                    self.tape[self.pointer] = buffer[0];
                }
                Instruction::Set { value } => {
                    self.tape[self.pointer] = *value;
                }
            }
        }
    }
}
