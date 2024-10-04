use std::{
    io::{Read, Write},
    slice,
};

pub struct State<'a> {
    pub input: Box<dyn Read + 'a>,
    pub output: Box<dyn Write + 'a>,
    pub tape: Box<[u8]>,
}

impl<'a> State<'a> {
    pub fn new(input: Box<dyn Read + 'a>, output: Box<dyn Write + 'a>, tape_size: usize) -> Self {
        State {
            input,
            output,
            tape: vec![0; tape_size].into_boxed_slice(),
        }
    }

    #[cfg(target_arch = "x86_64")]
    #[cfg(target_os = "windows")]
    pub unsafe extern "win64" fn getchar(state: &mut State, cell: *mut u8) -> u8 {
        Self::getchar_inner(state, cell)
    }

    #[cfg(target_arch = "x86_64")]
    #[cfg(target_os = "windows")]
    pub unsafe extern "win64" fn putchar(state: &mut State, cell: *mut u8) -> u8 {
        Self::putchar_inner(state, cell)
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub unsafe extern "sysv64" fn getchar(state: &mut State, cell: *mut u8) -> u8 {
        Self::getchar_inner(state, cell)
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub unsafe extern "sysv64" fn putchar(state: &mut State, cell: *mut u8) -> u8 {
        Self::putchar_inner(state, cell)
    }

    /// Reads a single byte from the input.
    unsafe fn getchar_inner(state: &mut State, cell: *mut u8) -> u8 {
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
    unsafe fn putchar_inner(state: &mut State, cell: *mut u8) -> u8 {
        match state.output.write_all(slice::from_raw_parts(cell, 1)) {
            Ok(_) => 0,
            Err(_) => 1,
        }
    }
}
