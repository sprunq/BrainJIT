use std::{
    io::{Read, Write},
    slice,
};

use super::TAPE_SIZE;

pub struct State<'a> {
    pub input: Box<dyn Read + 'a>,
    pub output: Box<dyn Write + 'a>,
    pub tape: [u8; TAPE_SIZE],
}

impl<'a> State<'a> {
    pub fn new(input: Box<dyn Read + 'a>, output: Box<dyn Write + 'a>) -> Self {
        State {
            input,
            output,
            tape: [0; TAPE_SIZE],
        }
    }

    /// Reads a single byte from the input.
    ///
    /// # Safety
    /// .
    pub unsafe extern "C" fn getchar(state: &mut State, cell: *mut u8) -> u8 {
        let mut buffer = [0];
        match state.input.read_exact(&mut buffer) {
            Ok(_) => {
                *cell = buffer[0];
                0
            }
            Err(_) => 1,
        }
    }

    /// Writes a single byte to the output.
    ///
    /// # Safety
    /// .
    pub unsafe extern "C" fn putchar(state: &mut State, cell: *mut u8) -> u8 {
        match state.output.write_all(slice::from_raw_parts(cell, 1)) {
            Ok(_) => 0,
            Err(_) => 1,
        }
    }
}
